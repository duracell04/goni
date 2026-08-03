---
id: GONI-IMAP-89E257E89BAF
title: 3.2 Internal API
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '` ust pub enum TaskClass { Interactive, Background, Maintenance, } pub struct ResourceBudget { pub max_prompt_tokens: usize, pub max_completion_tokens: usize, pub max_wall_time_ms: u64, } pub struct JobDescriptor { pub class: TaskClass, pub budget: ResourceBudget, pub model_hint: Option<ModelTier>, pub tools: Vec<ToolId>,'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 3.2 Internal API
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 3.2 Internal API

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Internal API

`
ust
pub enum TaskClass {
    Interactive,
    Background,
    Maintenance,
}

pub struct ResourceBudget {
    pub max_prompt_tokens: usize,
    pub max_completion_tokens: usize,
    pub max_wall_time_ms: u64,
}

pub struct JobDescriptor {
    pub class: TaskClass,
    pub budget: ResourceBudget,
    pub model_hint: Option<ModelTier>,
    pub tools: Vec<ToolId>,
    pub interaction_mode: InteractionMode,
    pub work_order_ref: Option<WorkOrderId>,
    pub user_profile: UserProfile,
    // opaque payload (prompt, metadata) lives in Arrow / ??
}
`

Control-plane interface:

`
ust
#[async_trait::async_trait]
pub trait ControlPlane {
    async fn submit(&self, job: JobDescriptor) -> JobHandle;
    async fn watch(&self, handle: &JobHandle) -> JobStatus;
    async fn cancel(&self, handle: &JobHandle) -> anyhow::Result<()>;
}
`

The Orchestrator only calls submit / watch / cancel and forwards streams back to the client.
It may surface `ReconstructionPreview` data, but that preview must come from
kernel-backed Work Order state rather than orchestration-local guesses.

Adjacent agent frameworks and workflow builders such as LangGraph, CrewAI,
AutoGen/AG2, Dify workflows, Flowise, LangFlow, n8n, LibreChat agents,
OpenClaw-like gateways, Open Interpreter, and MCP tool servers are tracked in
[Adjacent Projects](/blueprint/docs/adjacent-projects.md). They are useful
integration references, but they do not replace the Goni orchestrator/control
plane split or the kernel mediation boundary.

---
