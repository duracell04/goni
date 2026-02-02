# 20 – Orchestrator

Status: MVP – single-node orchestrator  
Scope: Logical routing inside one node (no mesh yet)

---

## 1. Role in the system

The **Orchestrator** is the front door and job submission layer:

- Accepts external requests (HTTP, CLI, IDE),
- Validates and normalises them,
- Builds **job descriptors**,
- Hands them to the Control Plane (??) for scheduling,
- Streams results back to the client.

It glues external APIs to the internal planes ?? (scheduler/router), ?? (context), and ?? (LLM runtime), without implementing their logic.

---

## 2. Responsibilities & non-responsibilities

### 2.1 Responsibilities

1. **Request ingestion**
   - HTTP /v1/chat/completions (OpenAI-like).
   - Local RPC/CLI entry points.

2. **Normalisation & validation**
   - Check request size against global limits (tokens, tools, attachments).
   - Map model="goni-small" ? internal ModelTier::Small.
   - Map request type to TaskClass (interactive/background).

3. **Job construction**
   - Build
     
     J = (\text{class}, \text{budget}, \text{tools}, \text{profile}, \dots)
     
     where udget encodes token/time/energy caps and class ? {interactive, background, maintenance}.

4. **Hand-off to Control Plane**
   - Submit J into scheduler queue.
   - Receive job completion / token stream from ??+??.

5. **Response handling**
   - Stream tokens back to clients.
   - Attach tool call traces / metadata.
   - Log metrics into the Data Plane (??) for observability.

### 2.2 Non-responsibilities

- ? Does *not* pick which model tier to use (router in ??).  
- ? Does *not* select RAG chunks (context selector in ??).  
- ? Does *not* run inference (LLM runtime in ??).  
- ? Does *not* enforce low-level resource limits (resman in ??/??).

---

## 3. Interfaces

### 3.1 External API (MVP)

HTTP /v1/chat/completions:

- Request: compatible with OpenAI’s chat/completions:
  - messages[], model, stream, optional 	ools.
- Response:
  - Non-streaming or server-sent events with tokens.

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

---

## 4. Invariants (tied to 30-conformance)

* **Admission invariant (K1)**
  The Orchestrator must apply request-level validation so that the global utilisation target in ?? is attainable, e.g. rejecting any single request whose budget would obviously violate
  
  \sum_i \lambda_i/\mu_i^{\max} < \alpha.
  

* **Plane separation invariant**
  All long-running work must enter through the Control Plane; the Orchestrator must *not* spin its own worker pools that bypass ??.

* **Local-first invariant**
  In offline mode, the Orchestrator may not introduce network dependencies; it just routes between local ??, ??, ??, ??.

---

## 5. MVP vs future cluster orchestrator

**MVP**

* Single node only.
* No cross-node routing.
* Simple mapping from HTTP request ? JobDescriptor.

**Future**

* Add a “target node” field on JobDescriptor and a Mesh layer that chooses nodes.
* Gateway orchestrator that can:

  * route interactive jobs to local node,
  * offload heavy jobs to remote nodes or cloud runtimes.

The Orchestrator spec here is the single-node kernel perspective; multi-node concerns live in mesh-and-wireguard.md.



## 6. Upstream
- [Job contract](../../docs/specs/job.md)
- [Scheduler and interrupts](../../docs/specs/scheduler-and-interrupts.md)
- [Tool capability API](../../docs/specs/tool-capability-api.md)
- [LLM runtime](./llm-runtime.md)

## 7. Downstream
- [API README](../../api/README.md)
- [OpenAPI spec](../../api/openapi.yaml)

## 8. Adjacent
- [Network gate and anonymity](../../docs/specs/network-gate-and-anonymity.md)
- [Receipts](../../docs/specs/receipts.md)
