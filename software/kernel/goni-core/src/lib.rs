use std::sync::Arc;

use goni_context::{record_batch_to_candidate_chunks, CandidateChunk, ContextSelector, KvPager};
use goni_infer::{LlmEngine, TokenStream};
use goni_router::{EscalationPolicy, Router};
use goni_sched::Scheduler;
use goni_store::DataPlane;
use goni_types::{ContextSelection, LlmRequest, TaskClass};

/// The orchestrator/kernel: wires the planes together.
pub struct GoniKernel {
    pub data_plane: Arc<dyn DataPlane>,
    pub context_selector: Arc<dyn ContextSelector>,
    pub kv_pager: Arc<dyn KvPager>,
    pub scheduler: Arc<dyn Scheduler>,
    pub router: Arc<dyn Router>,
    pub llm_engine: Arc<dyn LlmEngine>,
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
        }
    }

    /// High-level API: enqueue a query, produce a token stream.
    ///
    /// NOTE: For now this directly calls into llm_engine.
    /// In a full actor-based system, you'd:
    ///  - package this into a GoniBatch,
    ///  - submit to scheduler,
    ///  - have a separate loop driving scheduler.next() ? llm_engine.generate().
    pub async fn handle_user_query(
        &self,
        prompt: &str,
        _class: TaskClass,
    ) -> anyhow::Result<TokenStream> {
        // Placeholder embedding; swap in a real embedder.
        let emb_dim: usize = std::env::var("EMBED_DIM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1024);
        let query_embedding = vec![0.0_f32; emb_dim];
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
                (
                    ContextSelection {
                        indices: Vec::new(),
                        total_tokens: 0,
                    },
                    prompt.to_string(),
                )
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
