# 30 – Conformance Criteria & Proof Obligations (MVP Node)

Status: v0.1 (2025-11-30)
Scope: Single-node Goni kernel (no clustering)
Audience: Researchers, systems engineers, auditors

Normative contracts referenced by this document:

- Latent State Contract (LSC-01): `blueprint/30-specs/latent-state-contract.md`
- Tool syscall envelope (TOOL-01): `blueprint/30-specs/tool-capability-api.md`
- Agent/process model: `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Scheduler/interrupt semantics: `blueprint/30-specs/scheduler-and-interrupts.md`

> A **conformant Goni node** is one whose implementation can be mapped onto the four-plane model  
> $$
> N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})
> $$
> and that satisfies the invariants and proof obligations listed below, at least in an MVP / empirical sense.

---

## 1. Planes, objects, and invariants

We recap each plane and give:

- a **formal object** (what the plane *is*),  
- **invariants** (what must be true),  
- **proof obligations** (math-level),  
- **empirical checks** (MVP-level validation).

### 1.1 Data Plane \(\mathcal{A}\) – Arrow Spine

**Object.**  
A symmetric monoidal category
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
with:

- Objects: Arrow schemas \(S\) (finite products of fields).  
- Instances: Arrow `RecordBatch`es of schema \(S\).  
- Morphisms: affine, zero-copy transforms \(f : S \to T\) realised as total functions
  $$
  f^\# : \mathsf{Inst}(S) \to \mathsf{Inst}(T).
  $$

**Invariant A1 (zero-copy).**  
For every hot-path morphism \(f \in \mathcal{A}^{\text{hot}}\) and every batch \(B_S\),
$$
\Delta_\text{alloc}(f^\#, B_S)
= \bigl|\mathsf{Buf}(f^\#(B_S)) \setminus \mathsf{Buf}(B_S)\bigr|
= 0.
$$

**Invariant A2 (affine use).**  
Each payload buffer in \(\mathsf{Buf}(B_S)\) appears at most once in \(\mathsf{Buf}(f^\#(B_S))\); i.e. no “fan-out” of raw buffers on hot paths.

**Proof obligation (theoretical).**

1. Exhibit a set of primitive transforms \(\{f_i\}\) forming generators of \(\mathcal{A}^{\text{hot}}\).  
2. For each \(f_i\), argue from its construction that it reuses or slices only existing buffers.  
3. Show closure: composition and monoidal product of affine morphisms remain affine, hence A1 and A2 hold for all composites used on hot paths.

**Empirical check (MVP).**

- Instrument a small test harness that:
  - Wraps hot-path transforms,  
  - Counts allocations and buffer identities before/after.  
- Property-based test: for random batches up to some size,
  - Assert `?_alloc == 0` and no duplicate payload buffers in outputs.

A node **conforms** on the Data Plane if such tests pass and hot transforms are explicitly enumerated.

---

### 1.2 Context Plane \(\mathcal{X}\) – Submodular Context Selection

**Object.**  
A constrained monotone submodular maximisation problem over a ground set \(V\):

- Ground set \(V\) of chunks (top-K retrieved).  
- Cost \(c_i \in \mathbb{N}\) (token length) for each \(i \in V\).  
- Embeddings \(e_i \in \mathbb{R}^d\), query relevance scores \(r_i \ge 0\).  
- Objective:
  $$
  F(S) =
  \sum_{i \in V} \max_{j \in S} \cos(e_i, e_j)
  + \gamma \sum_{j \in S} r_j
  $$
- Constraint:
  $$
  \sum_{j \in S} c_j \le B.
  $$

**Invariant C1 (submodularity and guarantee).**

1. \(F\) is **monotone submodular** on \(2^V\).  
2. The implemented selector \(\mathsf{Select}\) uses greedy (or lazy greedy) and satisfies:
   $$
   F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^\*) - \varepsilon
   $$
   with \(\varepsilon\) controlled (target \(\varepsilon \le 10^{-6}\)).

**Proof obligation (theoretical).**

- Show that a single facility-location term \(\sum_{i} \max_{j\in S} k(i,j)\) with a non-negative similarity kernel \(k\) is monotone submodular.  
- Show that adding a non-negative modular term \(\gamma \sum_{j\in S} r_j\) preserves monotone submodularity.  
- Cite or reproduce the Nemhauser–Wolsey bound for greedy on monotone submodular maximisation under a knapsack or cardinality constraint.

**Empirical check (MVP).**

- On small problems (e.g. \(|V| \le 10\)):
  - Compute the exact optimum \(S^\*\) by brute force.  
  - Compute greedy solution \(S_{\text{greedy}}\).  
  - Assert
    $$
    F(S_{\text{greedy}}) \ge 0.63\, F(S^\*).
    $$
- Track ratio \(F_{\text{greedy}} / F_{\text{opt}}\) distribution across random synthetic instances.

A node **conforms** on the Context Plane if:

- The implemented objective is monotone submodular, and  
- Greedy meets (or empirically approximates) the \((1-1/e)\) bound on small instances.

---

### 1.3 Control Plane \(\mathcal{K}\) – Queueing & Router

**Object.**  
A controlled queueing network with:

- Three classes \(i \in \{1,2,3\}\) (interactive, background, maintenance).  
- Queue lengths \(Q_i(t)\), arrival rates \(\lambda_i\), max service rates \(\mu_i^{\max}\).  
- Priority weights \(w_i\).  
- Lyapunov function
  $$
  L(\mathbf{Q}) = Q_1^2 + 100 Q_2^2 + 10000 Q_3^2.
  $$

And a **router** that chooses between small and large models for each request.

#### 1.3.1 Scheduler: stability

**Invariant K1 (configured stability).**

- The admission policy ensures
  $$
  \sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < \alpha
  \quad\text{for some }\alpha < 1,
  $$
  with default \(\alpha = 0.94\).  
- The scheduler uses a MaxWeight-like policy:
  $$
  i^\*(t) = \arg\max_i \, w_i\,Q_i(t)\,\mu_i(t),
  $$
  where \(\mu_i(t)\) is an estimate of service rate.

**Proof obligation (theoretical).**

- State a standard Lyapunov drift condition:
  $$
  \mathbb{E}[L(\mathbf{Q}(t+1)) - L(\mathbf{Q}(t)) \mid \mathbf{Q}(t)] \le -\epsilon
  $$
  outside a finite set, for some \(\epsilon > 0\).  
- Argue that for \(\boldsymbol{\lambda}\) in the interior of the capacity region, MaxWeight satisfies such a condition (citing Tassiulas–Ephremides style results).  
- Conclude positive recurrence / stability of the queue process.

**Empirical check (MVP).**

- Implement a synthetic load simulator:
  - Poisson or bursty arrivals with total load tuned below \(\alpha\).  
  - Simple exponential or deterministic service times.  
- Show numerically:
  - \(\mathbb{E}[Q_i(t)]\) stabilises,  
  - \(\mathbb{E}[L(\mathbf{Q}(t))]\) does not diverge over long runs.  
- Optionally contrast with a naive scheduler to show exploding queues under similar load.

#### 1.3.2 Router: regret

**Invariant K2 (regret bound).**

Define:

- Policy \(\pi\) (router).  
- Oracle \(\pi^\*\) that knows which model (small vs large) yields best reward for each request.  
- Regret after \(T\) decisions:
  $$
  R_T = \sum_{t=1}^T \bigl( r(\pi^\*(x_t)) - r(\pi(x_t)) \bigr).
  $$

We require a bound of the form:
$$
\limsup_{T\to\infty} \frac{R_T}{T} \le \bar{R},
\quad\text{with target }\bar{R} \le 0.07.
$$

**Proof obligation (theoretical).**

- Model routing as a contextual bandit with two actions (small, large).  
- Assume a calibrated confidence estimator with bounded error \(\epsilon\).  
- Show that a threshold policy based on this confidence yields bounded average regret depending on \(\epsilon\) and cost/reward gaps.

**Empirical check (MVP).**

- Use a labelled dataset of requests with “ground-truth” best model decisions (e.g. preference data or accuracy labels).  
- Evaluate empirical regret of the router vs oracle:  
  $$
  \hat{R}_T/T \le 0.1
  $$
  as an initial MVP target.  
- Log this in CI to detect regressions.

A node **conforms** on the Control Plane if:

- Scheduler respects K1 (by construction + simulation).  
- Router exhibits bounded empirical regret on at least one non-trivial dataset.

---

### 1.4 Execution \(\mathcal{E}\) – Engines & Capabilities

**Object.**  
A family of models \(\mathcal{M}\) and tools running in an **effectful extension** of \(\mathcal{A}\):

- Engines \(m \in \mathcal{M}\) with capability descriptors.  
- Wasm modules \(W\) with capability sets \(\mathsf{Cap}(W)\).  
- An effectful category \(\mathcal{A}^\mathsf{eff}\) where side effects are annotated capabilities.

**Invariant E1 (capability safety).**

For any effectful morphism \(f_W\) implemented by a Wasm module \(W\),
$$
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W),
$$
and by default \(\mathsf{Cap}(W)\) is **local-only** (no network) for core tools.

**Invariant E2 (local-first).**

The core request?response function
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$
is total using only local state and compute; any network I/O is non-essential and opt-in.

**Proof obligation (theoretical).**

- Exhibit the host capability interface as a typed API.  
- Argue that all implemented tools / engines factor through this interface, so capability sets are sound.

**Empirical check (MVP).**

- Provide at least one **offline test configuration** where:
  - All network capabilities are disabled, yet  
  - Chat + RAG work against local models and local data.  
- Run that configuration in CI or as a nightly test.

**Invariant E3 (deterministic self-loop).**

When a request is marked deterministic, the engine uses the deterministic preset (single worker/thread, batch size 1, no continuous batching, TF32 off on NVIDIA, seed if supported). Empirical check:

- Run a fixed prompt in a self-loop for \(N\) steps (e.g. \(N=128\)) under the deterministic profile twice.
- Assert identical token streams (bitwise) and log the backend blueprint/hardware/driver version used.
- Fail conformance if drift appears.

---

## 2. MVP Conformance Checklist

A node qualifies as an **MVP-conformant Goni implementation** if:

### Data Plane

- [ ] Hot-path transforms are enumerated and documented as elements of \(\mathcal{A}^{\text{hot}}\).  
- [ ] Zero-copy invariant A1 is empirically tested on random batches.  
- [ ] Affine-use invariant A2 holds for all tested compositions on hot paths.

### Context Plane

- [ ] The objective \(F\) used by the selector is monotone submodular (facility location + modular term).  
- [ ] Greedy selection empirically achieves
  $$
  F(S_{\text{greedy}}) \ge 0.63\,F(S^\*)
  $$
  on small synthetic instances.

### Control Plane

- [ ] The scheduler uses a MaxWeight-like rule with a Lyapunov function \(L\) as documented.  
- [ ] Simulated workloads with load below \(\alpha\) show bounded queues (no drift to infinity).  
- [ ] The router is evaluated on labelled data and achieves average regret below a configured threshold (MVP: = 0.1).

### Execution & Capabilities

- [ ] All Wasm tools and engines declare capability sets; host enforces them.  
- [ ] There exists at least one configuration where all essential functionality runs without network access (local-first invariant).
- [ ] Deterministic preset passes the self-loop drift check (bitwise-stable tokens for fixed prompt; blueprint/hardware/driver versions logged).

When these conditions are met, we can credibly claim that a node realises the mathematical architecture of §20 and §95, even if the implementation is still minimal or unoptimised.


