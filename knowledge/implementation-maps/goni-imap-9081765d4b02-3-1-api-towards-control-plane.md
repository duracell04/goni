---
id: GONI-IMAP-9081765D4B02
title: 3.1 API towards Control Plane
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 3.1 API towards Control Plane
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 3.1 API towards Control Plane
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 3.1 API towards Control Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 API towards Control Plane

```rust
pub struct LlmRequest {
    pub bundle_id: BundleId,
    pub model_id: ModelId,
    pub prompt: PromptPlan,
    pub max_tokens: usize,
    pub session: Option<SessionId>,
}

pub struct ModelCapabilities {
    pub max_context: usize,
    pub tokens_per_second: f32,
    pub mem_bytes: u64,
    pub devices: Vec<DeviceKind>,
    pub speculative: Option<SpeculativeCapabilities>,
}

pub struct SpeculativeCapabilities {
    pub compatible_draft_bundles: Vec<BundleId>,
    pub max_draft_tokens: usize,
    pub reports_token_confidence: bool,
    pub verifier_batch_constraints: Vec<ShapeBucket>,
}

pub struct UtilizationMetrics {
    pub tokens_in_flight: u64,
    pub gpu_utilization: f32,   // [0,1]
    pub vram_bytes_used: u64,
}

pub struct ActiveBundle {
    pub bundle_id: BundleId,
    pub trunk_version: String,
    pub expert_mesh_version: String,
    pub patch_set_hashes: Vec<PatchHash>,
}
```

```rust
#[async_trait::async_trait]
pub trait LlmRuntime {
    async fn generate(
        &self,
        req: LlmRequest,
    ) -> anyhow::Result<TokenStream>;

    fn capabilities(&self, model_id: &ModelId) -> ModelCapabilities;

    fn utilization(&self) -> UtilizationMetrics;

    fn active_bundle(&self) -> ActiveBundle;
}
```
