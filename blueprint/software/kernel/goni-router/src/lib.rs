use async_trait::async_trait;
use goni_types::{ContextSelection, ModelTier};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

/// Router decisions should align with the escalation triggers in `docs/llm-council.md`
/// and the reference defaults in `config/council.yaml` (explicit user flag, high
/// difficulty/safety-critical classification, long-context needs).
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

#[derive(Clone, Debug)]
pub struct ConfigRouter {
    cfg: RouterConfig,
}

#[derive(Clone, Debug, Deserialize)]
struct RouterConfig {
    models: HashMap<String, ModelConfig>,
    routing: Option<RoutingConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct ModelConfig {
    id: String,
    max_tokens: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
struct RoutingConfig {
    local_first: Option<LocalFirstConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct LocalFirstConfig {
    target_local_token_ratio: Option<f32>,
}

impl ConfigRouter {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let text = fs::read_to_string(path)?;
        let cfg: RouterConfig = serde_yaml::from_str(&text)?;
        Ok(Self { cfg })
    }

    fn choose_tier(&self, prompt: &str) -> ModelTier {
        let default = ModelTier::LocalSmall;
        let Some(models) = self.cfg.models.get("interactive") else { return default; };
        let max_tokens = models.max_tokens.unwrap_or(256) as usize;
        if prompt.split_whitespace().count() > max_tokens {
            ModelTier::LocalLarge
        } else {
            default
        }
    }
}

#[async_trait]
impl Router for ConfigRouter {
    async fn decide(
        &self,
        prompt: &str,
        _context: &ContextSelection,
    ) -> (RoutingDecision, EscalationPolicy) {
        let tier = self.choose_tier(prompt);
        (
            RoutingDecision {
                chosen_tier: tier,
                reason: "config".into(),
            },
            EscalationPolicy::None,
        )
    }
}
