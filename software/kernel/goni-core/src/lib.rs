use std::sync::Arc;

use goni_context::{ContextSelector, KvPager};
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
        // TODO: integrate DataPlane + ContextSelector + Router properly.
        let context = ContextSelection {
            indices: Vec::new(),
            total_tokens: 0,
        };

        let (routing, _policy): (goni_router::RoutingDecision, EscalationPolicy) =
            self.router.decide(prompt, &context).await;

        let req = LlmRequest {
            prompt: prompt.to_string(),
            context,
            model_tier: routing.chosen_tier,
            max_tokens: 512,
        };

        let stream = self.llm_engine.generate(req).await?;
        Ok(stream)
    }
}
