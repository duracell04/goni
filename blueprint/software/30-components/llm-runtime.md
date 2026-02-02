# 80 – LLM Runtime

Status: MVP – unified local inference abstraction  
Scope: Single node; multiple models & backends via common API

---

## 1. Role in the system

The **LLM Runtime** is the Execution Plane (??) component that:

- Executes model inference for a given PromptPlan,
- Abstracts over concrete model backends (llama.cpp, vLLM, etc.),
- Exposes capabilities and utilisation back to the Control Plane (??).

It is the only component allowed to “speak” to GPUs/NPUs and LLM backends directly.

---

## 2. Responsibilities & boundaries

### 2.1 Responsibilities

- **Inference execution**
  - Convert a PromptPlan + ModelId into a token stream.

- **Model capability description**
  - Report per-model:
    - max_context,
    - nominal tokens/s,
    - memory footprint / device requirements,
    - supported shape buckets and graph-compile constraints (for NPUs),
    - KV-cache limits and paging mode (contiguous vs segmented).

- **Utilisation reporting**
  - Track and expose current load (per model and per device) to scheduler / resman in ??.
  - Expose effective bandwidth and memory pressure signals for routing decisions.

- **Cancellation / preemption hooks**
  - Support cooperative cancellation so ?? can abort or delay jobs.

- **Wake and warm-state control**
  - Report cold-start latency and warm state per model/device.
  - Support pre-warm and keep-alive budgets so decoder wake is bounded.
  - Support shape-bucket pre-warm for NPU graph stability.

### 2.2 Non-responsibilities

- ? Choosing which model tier to use (router).  
- ? Selecting RAG context (context selector).  
- ? Managing global queueing or admission control.

---

## 3. Interfaces

### 3.1 API towards Control Plane

`
ust
pub struct LlmRequest {
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
}

pub struct UtilizationMetrics {
    pub tokens_in_flight: u64,
    pub gpu_utilization: f32,   // [0,1]
    pub vram_bytes_used: u64,
}
`

`
ust
#[async_trait::async_trait]
pub trait LlmRuntime {
    async fn generate(
        &self,
        req: LlmRequest,
    ) -> anyhow::Result<TokenStream>;

    fn capabilities(&self, model_id: &ModelId) -> ModelCapabilities;

    fn utilization(&self) -> UtilizationMetrics;
}
`

### 3.2 Backend abstraction

Concrete engines (llama.cpp, vLLM, etc.) implement LlmRuntime:

* MVP: 1–2 backends is enough (e.g. local small/large model).

---

## 4. Invariants & performance targets

* **Capability invariant**
  ModelCapabilities must approximate real behaviour well enough that ??'s scheduling assumptions (capacity region) are not violated.

* **Shape-compatibility invariant**
  If a request's shape falls outside the backend's supported buckets, the runtime
  must route to a compatible device (CPU/iGPU/GPU) rather than padding or
  recompiling on the hot path.

* **Budget safety invariant**
  generate must not exceed max_tokens without explicit override.

* **Wake latency invariant**
  Time-to-first-token after a decoder wake must stay within the configured SLO; steady-state operation must not trigger implicit compilation or graph warmup.

* **Preemption invariant (soft)**
  Generation checks for cancellation at least once per decoding step (target preemption latency « human-visible 100 ms).

* **Streaming invariant**
  First token latency for interactive jobs stays within configured SLO (e.g. p99 < 1.0 s on reference hardware).

* **Deterministic preset**
  Runtime exposes a deterministic profile for audit/self-loop runs: temperature 0, fixed seed when backend supports it, batch size 1, no continuous/dynamic batching, single worker/thread (or CPU-only fallback), TF32 disabled on NVIDIA. Backend flags must be set accordingly (e.g. vLLM `--enable-deterministic-inference`; llama.cpp/Ollama single slot/thread), and the runtime logs device + driver hashes with each deterministic run.

---

## 5. MVP vs future runtime

**MVP**

* 1–2 local models (goni-small, goni-large).
* Single device type per session.
* No cross-session KV reuse beyond what backend provides.

**Future**

* Multi-device and multi-backend routing inside ??.
* Advanced KV cache paging tightly integrated with ??.
* Mixed local/cloud execution under the same interface.


## 6. Upstream
- [OS and base image](/blueprint/software/30-components/os-and-base-image.md)
- [Hardware requirements](/blueprint/software/hardware/10-requirements.md)

## 7. Downstream
- [Scheduler and interrupts](/blueprint/software/docs/specs/scheduler-and-interrupts.md)
- [ITCR](/blueprint/software/docs/specs/itcr.md)

## 8. Adjacent
- [Orchestrator](/blueprint/software/30-components/orchestrator.md)
- [Tool capability API](/blueprint/software/docs/specs/tool-capability-api.md)
