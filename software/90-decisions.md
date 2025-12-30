# 90 – Architecture & Implementation Decisions (Mathematical Form)

Status: Living document
Purpose: Record **deliberate design choices** in a way that makes their formal implications explicit.

---

## Index

- D-001 – Local-first, offline-capable by definition  
- D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)  
- D-003 – Affine, zero-copy morphisms in hot paths  
- D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))  
- D-005 – Single-node kernel, process structure is secondary  
- D-006 – Wasm as the extension substrate (effectful category)  
- D-007 – Submodular context selection with explicit bounds  
- D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter  
- D-009 – Small-then-big model routing with regret budget  
- D-010 – No implicit network morphisms (no hidden cloud)  
- D-011 – Strong default parameters; configuration bounded by invariants  
- D-012 - Monorepo + layered crate graph as DAG of modules  
- D-013 - Metrics as first-class objects in \(\mathcal{A}\)  
- D-014 - Invariants and theorems over ad-hoc tuning  
- D-015 - Deterministic inference preset for audit/self-loop workloads  
- D-016 - Memory Plane as a Pluggable, Local-First Service  
- D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)
- D-018 - Latent-first cognition as an architectural stance
- D-019 - Agents as local processes; solver as interrupt

---

## D-001 – Local-first, offline-capable by definition

**Formal statement**

A node \(N\) is *valid* iff the function
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$
is total and computable **using only local state and local compute**.

Equivalently: in the category of effectful computations \(\mathcal{A}^\mathsf{eff}\), any morphism used by the kernel must have an implementation that does not depend on remote network calls.

**Rationale**

- Eliminates remote-service availability as a factor in correctness.  
- Makes privacy and data-sovereignty constraints natural: all state lives in local objects of \(\mathcal{A}\).

**Consequence**

Any network effect \(e \in \mathsf{Effect}(\mathcal{A}^\mathsf{eff})\) is:

- Either outside the kernel (connectors, opt-in sync),  
- Or explicitly marked as “non-essential” (failure does not break \(\mathsf{Run}\) for local requests).

---

## D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)

**Formal statement**

All structured internal state is represented as objects of a single category:
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
(see 20-architecture) built on Arrow schemas and `RecordBatch`es.

Any new structured data type MUST:

- Be expressible as a schema \(S \in \mathrm{Ob}(\mathcal{A})\), and  
- Interact with other data via morphisms in \(\mathcal{A}\) or \(\mathcal{A}^\mathsf{eff}\).

**Rationale**

- Replaces a zoo of JSON/SQL/proto formats by a single, columnar algebra.  
- Ensures composability: any pipeline is a morphism in \(\mathcal{A}\) (or its effectful extension).

**Consequence**

- Code that manipulates structured data directly in ad-hoc formats is considered **non-conformant**.  
- For interoperability (e.g. with SQL), we define **functors**:
  $$
  F : \mathcal{A} \to \mathcal{B},
  $$
  e.g. to a relational category, instead of mutating foreign stores directly.

---

## D-003 – Affine, zero-copy morphisms in hot paths

**Formal statement**

We distinguish:

- \(\mathcal{A}^{\text{hot}}\): morphisms used on hot paths (per-request data handling).  
- \(\mathcal{A}^{\text{cold}}\): morphisms off hot paths (debug, export).

Constraint:
$$
\mathcal{A}^{\text{hot}} \subseteq \mathcal{A}_{rr}^{\text{affine}}.
$$

That is, for all \(f \in \mathcal{A}^{\text{hot}}\),
$$
\Delta_\text{alloc}(f^\#, B_S) = 0
\quad\forall B_S.
$$

**Rationale**

- Keeps memory and cache behaviour predictable.  
- Enables compositional reasoning: composition of hot-path transforms stays zero-copy.

**Consequence**

- When defining new transforms, we must categorise them as `hot` or `cold`.  
- CI includes property-based tests that check `hot` transforms allocate **no payload buffers**.

---

## D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))

**Formal statement**

The semantics of a node are factored as:
$$
\mathsf{Run} = F_{\mathcal{E}} \circ F_{\mathcal{K}} \circ F_{\mathcal{X}} \circ F_{\mathcal{A}}
$$
with:

- \(F_{\mathcal{A}}\) – retrieves/manipulates data purely via \(\mathcal{A}\).  
- \(F_{\mathcal{X}}\) – solves a submodular optimisation problem over retrieved chunks.  
- \(F_{\mathcal{K}}\) – schedules and routes work using \(\mathcal{K}\).  
- \(F_{\mathcal{E}}\) – executes engines / tools (\(\mathcal{E}\)).

**Rationale**

- Ensures there is **no direct path** from raw connectors to engines.  
- Each plane can have its own local invariants and be tested in isolation.

**Consequence**

- Architectural reviews must ask: “Which plane does this feature belong to?”  
- “Shortcut” code that calls models directly out of a connector is rejected by design.

---

## D-005 – Single-node kernel, process structure is secondary

**Formal statement**

We treat the node as a single abstract machine implementing:
$$
\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E}
$$
regardless of how many OS processes are involved.

The initial implementation uses **one process** (`goni-node`) that hosts all four components.

**Rationale**

- Separates logical semantics from OS deployment.  
- Keeps the minimal viable system simple (no distributed consensus, no multi-process scheduling).

**Consequence**

- Later multi-process / multi-node deployments must preserve the same semantics and invariants at the level of \(\mathcal{A},\mathcal{X},\mathcal{K},\mathcal{E}\), treating network boundaries as implementation details, not architectural ones.

---

## D-006 – Wasm as the extension substrate (effectful category)

**Formal statement**

Untrusted extensions (tools, agents, connectors) are **not** allowed to define morphisms directly in \(\mathcal{A}\). Instead they live in the effectful category \(\mathcal{A}^\mathsf{eff}\), where:

- Pure data transformations are still morphisms in \(\mathcal{A}\).  
- Side effects (I/O, network) are modelled as morphisms annotated with a capability set.

Let \(W\) be a Wasm module; we associate a capability set \(\mathsf{Cap}(W)\). For any effectful morphism \(f_W \in \mathcal{A}^\mathsf{eff}\) implemented by \(W\):
$$
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W).
$$

**Rationale**

- Makes safety properties (no arbitrary file/network access) explicit in the model.  
- Keeps the core spine (`goni-arrow`) free from unbounded side effects.

**Consequence**

- All extension APIs are mediated by a small, formally specified host interface.  
- Performance-critical extensions must be carefully designed to minimise \(\mathsf{Effects}\) and calls across the sandbox boundary.

---

## D-007 – Submodular context selection with explicit bounds

**Formal statement**

Context selection is always expressed as:
$$
\max_{S \subseteq V} F(S) \quad \text{s.t. } \sum_{i \in S} c_i \le B
$$
where \(F\) is **monotone submodular**, and solved using a greedy (or accelerated greedy) algorithm with known approximation guarantees (Theorem 2.2).

**Rationale**

- Gives a **lower bound** on the quality of the context we provide to models.  
- Provides a clear knob (\(\gamma\), \(B\)) for tuning diversity vs relevance.

**Consequence**

- Any proposal to change context selection must either:
  - Define a new submodular \(F'\) and maintain a similar bound, or  
  - Explicitly justify why we abandon mathematical guarantees.

---

## D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter

**Formal statement**

All work units (LLM calls, embeddings, indexing, compaction) are represented as jobs in the queueing network \(\mathcal{K}\). No component is allowed to maintain a “hidden” unbounded queue outside \(\mathcal{K}\).

Scheduling decisions are made exclusively by Policy K1 (MaxWeight) over classes 1–3.

**Rationale**

- Allows use of queueing theory to prove stability (Theorem 3.1).  
- Prevents “queueing inside the queue” anti-patterns that make latencies opaque.

**Consequence**

- Libraries like LLM engines or indexers must expose backpressure / job API so that their work can be scheduled centrally.  
- Introducing a new class of long-running task requires updating \(\mathcal{K}\) and its invariants, not spinning up an ad-hoc thread pool.

---

## D-009 – Small-then-big model routing with regret budget

**Formal statement**

We treat the routing problem as a decision between:

- \(a_s\): answer with small model only.  
- \(a_\ell\): escalate to large model.

We define a **regret budget** \(\bar{R}\), and require that:
$$
\limsup_{T \to \infty} \frac{R_T}{T} \le \bar{R}
$$
with \(\bar{R} = 0.07\) by default.

Here \(R_T\) is regret vs an oracle policy that knows the “true” best action per request.

**Rationale**

- Makes the “small-first” heuristic quantifiable: we know how much quality we trade for speed/cost.  
- Provides a clear metric for validating router training and calibration.

**Consequence**

- Router changes must be evaluated on standard corpora with regret estimates.  
- “Always large model” is allowed as a configuration but is explicitly outside the regret accounting (it corresponds to the oracle upper bound on quality, not the baseline).

---

## D-010 – No implicit network morphisms (no hidden cloud)

**Formal statement**

Any effectful morphism involving network I/O is explicitly annotated as such in \(\mathcal{A}^\mathsf{eff}\) and requires configuration.

Formally, for any \(f \in \mathcal{A}^\mathsf{eff}\):
$$
\text{if } \texttt{"network"} \in \mathsf{Effects}(f) \text{ then } f \text{ is opt-in and non-essential}.
$$

**Rationale**

- Aligns with D-001: local-first semantics.  
- Makes it trivial to inspect the code and see where data might leave the machine.

**Consequence**

- Even “harmless” things like version-check pings are explicitly implemented as such and can be disabled.  
- This simplifies compliance and audit (security review can focus on a small number of network-effect morphisms).

---

## D-011 – Strong default parameters; configuration bounded by invariants

**Formal statement**

We treat all tunable parameters (e.g. \(\gamma\), \(B\), scheduler weights, router thresholds) as living inside **safe regions** defined by invariants.

Example:

- Context plane: choose \(\gamma\) and \(B\) such that C1 holds and prompt budgets per model are respected.  
- Control plane: choose admission thresholds so that K1 holds.

Parameters outside these safe regions are allowed only in “experimental” modes.

**Rationale**

- Keeps default nodes in the regime where our theorems apply.  
- Makes configuration safer: users can change things without accidentally destroying stability.

**Consequence**

- Config parsing includes validation against invariant ranges.  
- Documentation describes both: (a) default values, (b) safe ranges implied by theory.

---

## D-012 – Monorepo + layered crate graph as DAG of modules

**Formal statement**

The core project is a monorepo whose crate dependency graph is a **directed acyclic graph (DAG)**:

- There exists a partial order \(\prec\) on crates such that if crate \(A\) depends on crate \(B\), then \(B \prec A\).  
- “Lower” crates are closer to the math (Arrow, scheduler); “higher” crates implement user-facing behaviour.

**Rationale**

- Acyclic graph reflects the mathematical layering: \(\mathcal{A} \to \mathcal{X} \to \mathcal{K} \to \mathcal{E} \to \text{UI}\).  
- Simplifies reasoning about where invariants are enforced (at the bottom of the DAG).

**Consequence**

- Introducing a dependency cycle is considered a structural bug; CI rejects it.  
- Cross-cutting functionality (e.g. tracing) must be injected via interfaces, not by making everything depend on everything.

---

## D-013 – Metrics as first-class objects in \(\mathcal{A}\)

**Formal statement**

All metrics, logs and traces are represented as objects in \(\mathcal{A}\):

- There exists a schema \(S_\text{log}\) for logs, \(S_\text{metric}\) for metrics, etc.  
- Emission of metrics is a morphism:
  $$
  \mathsf{Emit} : S \to S \oplus S_\text{metric}
  $$
  in \(\mathcal{A}^{\text{cold}}\).

**Rationale**

- Makes metrics queryable using the same columnar tooling as user data.  
- Enables offline, local analysis of behaviour without relying on external telemetry.

**Consequence**

- We must define stable schemas for these “meta” objects and version them.  
- External observability systems (Prometheus, OTLP, etc.) are treated as sinks fed from \(\mathcal{A}\), not as authoritative stores.

---

## D-014 – Invariants and theorems over ad-hoc tuning

**Formal statement**

For each major subsystem we choose a set of **invariants** and/or **theorems**, and treat them as part of the public contract:

- Data Plane: A1, A2.  
- Context Plane: C1.  
- Control Plane: K1, K2.  
- Execution substrate: E1.

We then require that:

1. CI includes tests or simulations that exercise these invariants.  
2. “Optimisations” that would break an invariant are not allowed in stable releases.

**Rationale**

- We want Goni to be a **kernel with proofs**, not just a performing demo.  
- Invariants help future contributors understand what they may and may not change.

**Consequence**

- Some micro-optimisations that improve a single benchmark but violate zero-copy or stability constraints are rejected.  
- Changes to invariants go through an explicit "amendment" process in this document (with versioning and rationale), so we keep a history of our mathematical commitments.

---

## D-015 - Deterministic inference preset for audit/self-loop workloads

**Formal statement**

The Execution plane exposes a deterministic preset. For any request marked deterministic:

- temperature = 0 and fixed seed (if supported by backend),  
- batch size = 1 with no continuous/dynamic batching,  
- single worker/thread (or CPU-only fallback) and TF32 disabled on NVIDIA,  
- deterministic backend flag enabled (e.g. vLLM `--enable-deterministic-inference`),  
- hardware/driver hashes recorded with the run.

**Rationale**

- Self-loop/agent chains have positive Lyapunov exponents; tiny numeric noise can flip tokens and diverge trajectories.  
- Regulated or audited runs must be reproducible even at the cost of throughput.

**Consequence**

- Engines must provide a slower deterministic profile rather than silently ignoring the request.  
- CI includes a self-loop drift check (bitwise-stable tokens across two runs) under the deterministic preset.  
- Fast defaults may use batched/GPU paths, but the audit preset remains available and documented.

---

## D-016 - Memory Plane as a Pluggable, Local-First Service

**Formal statement**

Long-term memory is modelled as a separate **Memory Plane** with a stable API (`store`, `recall`, `forget`, `summarize`, `audit`). Engines and controllers access memory only through this plane; core LLMs stay stateless. The default backend is Arrow tables + vector index (Qdrant/Lance), and backends are **swappable** (e.g. OpenMemory/Mem0/graph/curved indexes) without changing \((\mathcal{E})\).

**Rationale**

- Keeps reasoning and memory decoupled; enables backend experimentation without kernel surgery.  
- Aligns with privacy/local-first: long-term memory stays on-device; council/cloud paths see at most distilled facts or session context by explicit choice.  
- Supports lifecycle (working/episodic/semantic/procedural) with decay/pin/forget and auditability.

**Consequence**

- Kernel code and tooling MUST use the Memory Plane interface rather than ad-hoc embedding stores.  
- Forget/redaction and audit traces are first-class behaviours of the plane.  
- Backend swaps must preserve the API contract and lifecycle semantics; otherwise the decision must be amended.

---

## D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)

**Formal statement**

- The Memory/Context planes MUST implement **virtual context management** at the kernel level:
  - Treat the prompt window as RAM and external stores (Arrow spine + vector/graph backends) as Disk.
  - Expose explicit paging/syscalls (e.g. `MEM_READ`, `MEM_WRITE`, `MEM_SUMMARIZE`, `MEM_FORGET`) to move data across tiers.
  - Keep LLM engines stateless; all long-lived state flows through the Memory Plane.
- The Control Plane MUST schedule a recurring **Observation → Reflection → Planning** consolidation loop:
  - ingest raw events into episodic memory (Observation),
  - distill reflections/long-term facts (Reflection),
  - produce plans/jobs/actions using both current state and reflections (Planning).
- Council/cloud paths see only distilled/approved context unless explicitly allowed by policy.

**Rationale**

- MemGPT (Packer et al., 2023) formalises virtual context management; Goni adopts it as a kernel invariant rather than a prompt-only tool.
- Generative Agents (Park et al., 2023) shows Observation–Reflection–Planning is necessary for coherent long-horizon behaviour; we bake this into the Control/Memory planes.

**Consequence**

- Kernel APIs must surface paging syscalls to tools/agents; backlog item: expose `MEM_*` calls in the extension substrate.
- Context selection and memory paging are tested as first-class behaviours; prompt-only hacks are non-conformant.
- Nightly/periodic consolidation jobs become required workloads; they must respect policies and budgets (local-first unless configured otherwise).

---

## D-018 - Latent-first cognition as an architectural stance

**Status:** Proposed  
**Date:** YYYY-MM-DD

**Formal statement**

We adopt **latent-first cognition** as a guiding stance at the architecture/interface level:

- Maintain a latent "world state" as the primary internal representation.  
- Treat language generation as an optional downstream rendering step.  
- Represent retrieval (RAG) as a tool invoked by the predictor, not as the cognitive core.  
- Allow multiple encoders/decoders to interoperate through a stable latent-state contract.

**Rationale**

- Local-first efficiency: always-on components can be smaller than full decoders.  
- Privacy boundaries: avoid unnecessary raw text duplication.  
- Tool-first execution: decisions precede narration.  
- Modular interfaces: swap encoders/decoders without changing the Control Plane.

**Consequence**

- Pros: lower always-on compute, cleaner tool routing, and better separation of state vs narration.  
- Cons/risks: latent state is harder to inspect; embedding collapse needs evaluation and guardrails.  
- This ADR defines an architectural stance, not a mandatory training objective.

---

## D-019 - Agents as local processes; solver as interrupt

**Status:** Proposed  
**Date:** YYYY-MM-DD

**Formal statement**

Agents are modeled as userland processes that operate on kernel-owned latent
state and invoke capability-scoped syscalls. LLM/solver execution is a
budgeted interrupt, not a control loop:
$$
\text{agent step} \Rightarrow \mathsf{read\_state} \to \mathsf{tool}^\* \to \mathsf{commit},
$$
and
$$
\mathsf{solver} \text{ is invoked iff } \text{interrupt\_condition} = \text{true}.
$$

**Rationale**

- Aligns with local-first power/thermal constraints.  
- Makes policy mediation and auditability explicit.  
- Prevents hidden LLM loops that burn budgets and drift state.

**Consequence**

- All agent effects must be routed through capability tokens and audit records.  
- Scheduler must enforce wake hysteresis and solver budgets.  
- Kernel APIs define the single interface for state access and commits.

*Amendment process:*  
New decisions should include:

- A short **formal statement** (equation, inequality, category-theoretic object, etc.).  
- A rationale explaining why the formal constraint is desirable.  
- Consequences for implementation and testing.

Changes to existing decisions are logged with a date and an explanation of why the previous constraint was no longer adequate.

