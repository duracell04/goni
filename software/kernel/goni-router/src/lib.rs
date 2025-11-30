use async_trait::async_trait;
use goni_types::{ContextSelection, ModelTier};

#[derive(Clone, Debug)]
pub struct RoutingDecision {
    pub chosen_tier: ModelTier,
    pub reason: String,
}

/// Optional escalation policy (e.g., for SPRT-style mid-generation escalation).
#[derive(Clone, Debug)]
pub enum EscalationPolicy {
    None,
    SprtThreshold(f32),
}

#[async_trait]
pub trait Router: Send + Sync {
    async fn decide(
        &self,
        prompt: &str,
        context: &ContextSelection,
    ) -> (RoutingDecision, EscalationPolicy);
}

/// Dummy implementation: always use LocalSmall.
pub struct NullRouter;

#[async_trait]
impl Router for NullRouter {
    async fn decide(
        &self,
        _prompt: &str,
        _context: &ContextSelection,
    ) -> (RoutingDecision, EscalationPolicy) {
        (
            RoutingDecision {
                chosen_tier: ModelTier::LocalSmall,
                reason: "NullRouter".into(),
            },
            EscalationPolicy::None,
        )
    }
}
