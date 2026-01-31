use std::{collections::HashMap, sync::Arc};

use goni_context::{record_batch_to_candidate_chunks, CandidateChunk, ContextSelector, KvPager};
use goni_infer::{LlmEngine, TokenStream};
use goni_router::{EscalationPolicy, Router};
use goni_sched::Scheduler;
use goni_store::DataPlane;
use goni_types::{ContextSelection, LlmRequest, TaskClass};

use tokio::sync::{oneshot, Mutex};
use uuid::Uuid;

struct PendingRequest {
    prompt: String,
    class: TaskClass,
    tx: oneshot::Sender<anyhow::Result<TokenStream>>,
}

/// The orchestrator/kernel: wires the planes together.
pub struct GoniKernel {
    pub data_plane: Arc<dyn DataPlane>,
    pub context_selector: Arc<dyn ContextSelector>,
    pub kv_pager: Arc<dyn KvPager>,
    pub scheduler: Arc<dyn Scheduler>,
    pub router: Arc<dyn Router>,
    pub llm_engine: Arc<dyn LlmEngine>,

    /// Requests waiting to be executed by the scheduler loop.
    ///
    /// Key: batch_id (scheduler meta id)
    pending: Mutex<HashMap<Uuid, PendingRequest>>,
}

impl GoniKernel {
    pub fn new(
        data_plane: Arc<dyn DataPlane>,
        context_selector: Arc<dyn ContextSelector>,
        kv_pager: Arc<dyn KvPager>,
        scheduler: Arc<dyn Scheduler>,
        router: Arc<dyn Router>,
        llm_engine: Arc<dyn LlmEngine>,
    ) -> Self {
        Self {
            data_plane,
            context_selector,
            kv_pager,
            scheduler,
            router,
            llm_engine,
            pending: Mutex::new(HashMap::new()),
        }
    }

    /// High-level API: enqueue a query and await the solver result.
    ///
    /// Important: this method does **not** call the LLM directly.
    /// The LLM is invoked by the scheduler executor loop (see `run_scheduler_loop`).
    pub async fn handle_user_query(&self, prompt: &str, class: TaskClass) -> anyhow::Result<TokenStream> {
        let (batch_id, rx) = self.submit_user_query(prompt, class).await?;
        // In MVP, if the executor loop is not running, run one step inline.
        // This keeps CLI/dev usage working while preserving the architectural boundary.
        if self.pending.lock().await.contains_key(&batch_id) {
            self.run_scheduler_once().await;
        }
        rx.await?
    }

    /// Submit a user query into the scheduler and return a oneshot that yields the token stream.
    pub async fn submit_user_query(
        &self,
        prompt: &str,
        class: TaskClass,
    ) -> anyhow::Result<(Uuid, oneshot::Receiver<anyhow::Result<TokenStream>>)> {
        let batch_id = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending.lock().await;
            pending.insert(
                batch_id,
                PendingRequest {
                    prompt: prompt.to_string(),
                    class,
                    tx,
                },
            );
        }

        // Submit a minimal batch (payload-free for MVP). Scheduler sees meta only.
        let schema = Arc::new(arrow::datatypes::Schema::empty());
        let empty = arrow::record_batch::RecordBatch::new_empty(schema);
        let batch = goni_types::GoniBatch {
            data: Arc::new(empty),
            meta: goni_types::BatchMeta {
                id: batch_id,
                class,
                arrival_ts: std::time::Instant::now(),
                est_tokens: prompt.split_whitespace().count().max(1),
            },
        };
        self.scheduler
            .submit(batch)
            .await
            .map_err(|e| anyhow::anyhow!(e.message))?;

        Ok((batch_id, rx))
    }

    /// Run one scheduled batch (if any) and deliver its result to the waiting receiver.
    pub async fn run_scheduler_once(&self) {
        let Some(batch) = self.scheduler.next().await else { return; };
        let pending = {
            let mut p = self.pending.lock().await;
            p.remove(&batch.meta.id)
        };
        let Some(req) = pending else { return; };

        let result = self.solve_prompt(&req.prompt, req.class).await;
        let _ = req.tx.send(result);
    }

    /// Background executor loop. Call this once from the embedding host (HTTP server / daemon).
    pub async fn run_scheduler_loop(self: Arc<Self>) {
        loop {
            self.run_scheduler_once().await;
            // MVP: cooperative yield to avoid a busy loop.
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }

    async fn solve_prompt(&self, prompt: &str, _class: TaskClass) -> anyhow::Result<TokenStream> {
        // Deterministic lexical embedding baseline.
        let emb_dim: usize = std::env::var("EMBED_DIM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1024);
        let query_embedding = goni_embed::embed(prompt, emb_dim);
        let collection = std::env::var("QDRANT_COLLECTION").unwrap_or_else(|_| "default".into());

        // Fetch candidates from the data plane (Qdrant-backed when configured).
        let rag_batch = self
            .data_plane
            .rag_candidates(&collection, &query_embedding, 128)
            .await;

        let (context, augmented_prompt) = match rag_batch {
            Ok(batch) => {
                let candidates: Vec<CandidateChunk> = match record_batch_to_candidate_chunks(
                    &batch,
                    "id",
                    "tokens",
                    "embedding",
                    &query_embedding,
                ) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("context build error: {e:?}");
                        Vec::new()
                    }
                };

                let selection = self
                    .context_selector
                    .select(&query_embedding, &candidates, 2048)
                    .await;

                // Append selected context text to prompt if available.
                let mut ctx_block = String::new();
                for idx in &selection.indices {
                    if let Some(chunk) = candidates.get(*idx as usize) {
                        if let Some(text) = chunk.text {
                            ctx_block.push_str("- ");
                            ctx_block.push_str(text);
                            ctx_block.push('\n');
                        } else {
                            ctx_block.push_str("- ");
                            ctx_block.push_str(chunk.id);
                            ctx_block.push('\n');
                        }
                    }
                }

                let aug_prompt = if ctx_block.is_empty() {
                    prompt.to_string()
                } else {
                    format!("{}\n\nContext:\n{}", prompt, ctx_block)
                };

                (selection, aug_prompt)
            }
            Err(e) => {
                eprintln!("rag_candidates error: {e:?}");
                let demo = std::env::var("GONI_DEMO_CONTEXT")
                    .map(|v| v == "1" || v.to_lowercase() == "true")
                    .unwrap_or(false);
                if demo {
                    let demo_text = "demo context";
                    let demo_emb = goni_embed::embed(demo_text, emb_dim);
                    let candidates = vec![CandidateChunk {
                        id: "demo",
                        text: Some(demo_text),
                        tokens: 1,
                        embedding: &demo_emb,
                        relevance: 1.0,
                    }];
                    let selection = self
                        .context_selector
                        .select(&query_embedding, &candidates, 64)
                        .await;
                    let aug_prompt = format!("{}\n\nContext:\n- {}", prompt, demo_text);
                    (selection, aug_prompt)
                } else {
                    (
                        ContextSelection {
                            indices: Vec::new(),
                            total_tokens: 0,
                        },
                        prompt.to_string(),
                    )
                }
            }
        };

        let (routing, _policy): (goni_router::RoutingDecision, EscalationPolicy) =
            self.router.decide(&augmented_prompt, &context).await;

        let req = LlmRequest {
            prompt: augmented_prompt,
            context,
            model_tier: routing.chosen_tier,
            max_tokens: 512,
        };

        let stream = self.llm_engine.generate(req).await?;
        Ok(stream)
    }
}
