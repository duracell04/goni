# 20 â€“ Software Architecture (Formal View)

Status: v0.1 (2025-11-30)
Audience: Systems & ML engineers, PL / formal methods people
Scope: Single Goni node (laptop / workstation / edge box)

---

## 0. Notation and overview

A **Goni node** is modelled as a 4-tuple
$$
N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})
$$
where:

- \(\mathcal{A}\): **Arrow Spine** â€“ a symmetric monoidal category of zero-copy data transforms.
- \(\mathcal{X}\): **Context Plane** â€“ a submodular optimisation problem over retrieved chunks.
- \(\mathcal{K}\): **Control Plane** â€“ a controlled queueing network with a Lyapunov scheduler.
- \(\mathcal{E}\): **Execution Substrate** â€“ LLM / embed engines + Wasm sandboxes.

We use this decomposition both as an implementation guide and as the basis for our invariants.

**How to read this doc.**

- This file defines the architecture and its formal objects.
- `blueprint/software/30-conformance.md` turns those objects into invariants and proof obligations (what must be shown or tested for an MVP node to be â€œconformantâ€).
- `blueprint/software/95-theory-appendix.md` gives a brief theoretical backdrop (category theory, submodularity, Lyapunov stability, bandits, capabilities).

Read in the order: **Architecture (20) -> Conformance (30) -> Theory (95)**.

---

## 0.1 Memory Plane (operational abstraction)

While the formal tuple remains \(N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})\), we treat **long-term memory as an external, pluggable service** that the Control/Execution planes call:

- **API surface:** `store(entry)`, `recall(query, mode)`, `forget(selector)`, `summarize(range)`, `audit(entry_id)`.
- **Types / lifecycle:** `working` (session-scoped), `episodic` (raw events distilled over time), `semantic` (facts with decay/pin), `procedural` (versioned skills/tools). States move `raw -> distilled -> archived/tombstoned`, with decay on importance/access.
- **Local-first:** long-term memory lives on-device (Arrow + vector/graph backend). Council/cloud paths see only distilled facts or session context unless explicitly allowed.
- **Reasoning statelessness:** LLM engines stay stateless; they access memory exclusively through this plane so backends can be swapped (Qdrant/Arrow today; OpenMemory/Mem0/curved indexes later) without changing \(\mathcal{E}\).

---

## 0.2 Practical architecture (rings + interrupts)

Goni OS treats agents as **local userland processes** and the LLM as a **rare,
budgeted interrupt**, not a control loop. The practical architecture is a
three-ring model:
This structure is what makes Goni a "digital double": a local process that
observes, distills, and acts under explicit policy and receipts.

- **Ring 0 (Cognitive kernel):** observation bus, latent state store, policy
  engine, scheduler/interrupt controller.
- **Ring 1 (Always-on cognition):** encoders + predictor update latent state,
  compute surprisal/goal drift, and decide whether to raise interrupts.
- **Ring 2 (Userland):** agent runtime and solver/decoder services invoked on
  demand through kernel APIs.

Canonical specs:

- Latent state contract: `blueprint/30-specs/latent-state-contract.md`
- Agents and manifests: `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Tools and audit: `blueprint/30-specs/tool-capability-api.md`
- Scheduler/interrupts: `blueprint/30-specs/scheduler-and-interrupts.md`
- ITCR cascade: `blueprint/30-specs/itcr.md`

This section is a practical view; the formal planes \((\mathcal{A}, \mathcal{X},
\mathcal{K}, \mathcal{E})\) below define the invariants.

### 0.2.1 ITCR cascade (asymmetric, gated)

Goni treats inference-time compute reasoning (ITCR) as a bounded, interrupt-
driven service. The default loop is low-power, and ITCR is activated only when
escalation predicates indicate that extra compute is worth the cost.

Stages:

1) Continuous state maintenance (encoders + predictor).
2) Cheap proposer (small model or heuristic plan).
3) Escalation policy (explicit predicates, hysteresis).
4) ITCR reasoner/verifier (bounded search + repair loop).
5) Commit under governance (policy validation + audit).

See `blueprint/30-specs/itcr.md` for budgets, triggers, and invariants.

---

## 1. Data Plane - the Arrow Spine \(\mathcal{A}\)

### 1.1 Objects and morphisms

We define a symmetric monoidal category
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
whose objects are Arrow schemas and whose morphisms are **affine, zero-copy transforms** between Arrow `RecordBatch`es.

- **Objects.**  
  An object \(S \in \mathrm{Ob}(\mathcal{A})\) is a finite, ordered product of Arrow fields:
  $$
  S = (f_1 : \tau_1, \dots, f_n : \tau_n)
  $$
  and is represented in code as `SchemaRef`.

- **Instances.**  
  For each schema \(S\), the set of instances \(\mathsf{Inst}(S)\) is the set of Arrow `RecordBatch`es whose schema is \(S\).  
  In practice, an instance is a tuple of `ArrayData` values
  $$
  B_S = (a_1, \dots, a_n), \quad a_i \in \text{ArrayData}(\tau_i)
  $$
  each backed by one or more buffers \(b \in \text{Buffer} \cong \text{Arc<[u8]>}\).

- **Morphisms.**  
  A morphism \(f : S \to T\) is implemented as a total function:
  $$
  f^\# : \mathsf{Inst}(S) \to \mathsf{Inst}(T)
  $$
  such that:

  1. (**Affine use**) each input buffer is used in constructing at most one output buffer; we never â€œfan-outâ€ raw buffers.  
  2. (**Zero-copy**) any new `ArrayData` is built exclusively from:
     - existing buffers via slice (`offset`, `len`), or  
     - new **metadata only** (offsets/lengths, validity bitmaps) but **no new payload buffers**.

Formally, let \(\mathsf{Buf}(B)\) be the multiset of payload buffers of a batch \(B\). Define
$$
\Delta_\text{alloc}(f^\#, B_S) \equiv \bigl|\mathsf{Buf}(f^\#(B_S)) \setminus \mathsf{Buf}(B_S)\bigr|
$$
(counting only newly allocated payload buffers, not metadata).

> **Definition 1 (Affine zero-copy morphism).**  
> A morphism \(f^\#: \mathsf{Inst}(S) \to \mathsf{Inst}(T)\) is in \(\mathcal{A}_{rr}^{\text{affine}}\) iff, for all inputs \(B_S\),
> $$
> \Delta_\text{alloc}(f^\#, B_S) = 0
> \quad\text{and}\quad
> \text{each } b \in \mathsf{Buf}(B_S) \text{ appears at most once in } \mathsf{Buf}(f^\#(B_S)).
> $$

This is enforced in code by constraining transforms to:

- borrow slices (`ArrayData::new` with existing `Arc<Buffer>`), and  
- reject transforms that construct new payload `Buffer`s on hot paths.

### 1.2 Monoidal structure

The monoidal product of \(\mathcal{A}\) is **schema concatenation**:

- On objects:
  $$
  S \oplus T = (f_1:\tau_1, \dots, f_m:\tau_m, g_1:\sigma_1, \dots, g_k:\sigma_k)
  $$
- On instances:
  $$
  B_S \otimes B_T \coloneqq \texttt{RecordBatch::try\_new}(S\oplus T, [a_1,\dots,a_m,b_1,\dots,b_k])
  $$
  which is again zero-copy because we only re-use `ArrayData` handles.

> **Invariant A1 (monoidal zero-copy).**  
> For all \(f, g \in \mathcal{A}_{rr}^{\text{affine}}\),
> $$
> \Delta_\text{alloc}(f \otimes g,\, B_S \otimes B_T) = 0.
> $$

The unit object is the empty schema \(I \equiv ()\), represented as `Schema::empty()`.

### 1.3 Relation to linear algebra and relations

Ignoring nullability and offsets, each column of a batch can be regarded as an element of a finite-dimensional vector space over \(k = \mathbb{R}\) or \(\mathbb{Q}\), and a `RecordBatch` as an object in \(\mathrm{FinVect}_k\).

For subsystems with feedback and constraints (e.g. data-dependent scheduling decisions), it is more appropriate to work in the category of **linear relations** \(\mathrm{FinRel}_k\):

- A relation \(R \subseteq U \times V\) models partial, multi-valued transforms.  
- Caps and cups (trace operators) model feedback loops.

At the architecture level we require:

> **Invariant A2 (realizability).**  
> Every linear relation \(R : S \rightsquigarrow T\) used in a dataflow graph must admit an **implementation** as a composite of affine morphisms in \(\mathcal{A}_{rr}^{\text{affine}}\) plus a finite set of **materialisation points** where we explicitly allow allocation.

This lets us draw diagrams in a richer relational calculus, while pinning hot paths to zero-copy implementations.

### 1.4 Implementation mapping

- Crate `goni-arrow` implements \(\mathcal{A}_{rr}^{\text{affine}}\).  
- Crate `goni-store` provides persistent functors:
  $$
  \mathrm{Persist} : \mathcal{A} \to \mathcal{A}
  $$
  that map in-memory batches to on-disk segments (Parquet/Lance) and back.  
- Crate `goni-index` provides indexed projections:
  $$
  P : \mathcal{A} \to \mathcal{A}, \quad \text{e.g. } (S \mapsto S') \text{ where } S' \text{ only keeps chunk id + embedding}.
  $$

---

## 2. Context Plane \(\mathcal{X}\) â€“ RAG as submodular optimisation

The Context Plane chooses which pieces of data (chunks) to show to a model, subject to a token budget.

### 2.1 Ground set and features

Let:

- \(V\) be the set of candidate chunks for a query (top-\(K\) from ANN, typically \(K=512\)).  
- For each \(i \in V\), we store:
  - embedding \(e_i \in \mathbb{R}^d\) (normalised, \(d = 1024\)),  
  - cost \(c_i \in \mathbb{N}\) (token length),  
  - query relevance score \(r_i \in \mathbb{R}_{\ge 0}\) (e.g. cosine similarity to query embedding \(q\)).

Let \(B\) be a token budget (prompt + context).

### 2.2 Objective and optimisation problem

We define the **facility location + relevance** objective:
$$
F(S) =
\underbrace{
\sum_{i \in V} \max_{j \in S} \cos(e_i, e_j)
}_{\text{coverage term}} +
\gamma \underbrace{\sum_{j \in S} r_j}_{\text{relevance term}}
\quad \text{for } S \subseteq V,
$$
with trade-off parameter \(\gamma \ge 0\).

This induces a constrained maximisation problem:
$$
\max_{S \subseteq V}
\quad F(S)
\quad \text{s.t.} \quad \sum_{j \in S} c_j \le B.
$$

> **Proposition 2.1.**  
> The function \(F : 2^V \to \mathbb{R}_{\ge 0}\) is **monotone submodular**.  
> (Monotone: \(F(S) \le F(T)\) when \(S \subseteq T\). Submodular: diminishing returns.)

### 2.3 Algorithm and guarantee

We implement **lazy greedy**:

1. Initialise \(S_0 = \varnothing\).  
2. At step \(t\), for each \(j \in V \setminus S_t\), compute marginal gain:
   $$
   \Delta(j \mid S_t) = F(S_t \cup \{j\}) - F(S_t).
   $$
3. Choose \(j^\* = \arg\max_j \Delta(j \mid S_t)/c_j\) while \(\sum_{k \in S_t} c_k + c_{j^\*} \le B\).  
4. Set \(S_{t+1} = S_t \cup \{j^\*\}\).  
5. Stop when no item fits.

> **Theorem 2.2 (Approximation bound).**  
> Let \(S^\*\) be an optimal solution and \(S_{\text{greedy}}\) the output of lazy greedy. Then:
> $$
> F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^\*) - \varepsilon
> $$
> for \(\varepsilon\) bounded by numerical precision and the stopping criterion. In practice we target \(\varepsilon \le 10^{-6}\).

> **Invariant C1 (Context guarantee).**  
> For every query,
> $$
> \frac{F(S_{\text{greedy}})}{F(S^\*)} \ge 1 - 1/e - \delta
> $$
> where \(\delta\) is tracked as a runtime statistic and must stay \(< 0.03\) in regression tests.

### 2.4 Determinism and reproducibility

For a fixed snapshot of the Data Plane (fixed embeddings, fixed ANN retrieval order), the context selection must be **deterministic**.

Formally, there exists a pure function
$$
\mathsf{Select} : (q, V, B) \mapsto S \subseteq V
$$
such that repeated calls with the same arguments yield the same selected set \(S\).

Implementation constraints:

- `goni-context` uses only deterministic operations; random tiebreakers are derived from stable chunk IDs.  
- Budget and similarity calculations are pure functions of inputs.

---

## 3. Control Plane \(\mathcal{K}\) â€“ queueing network and scheduler

### 3.1 Work classes and queues

We model the node as a discrete-time (or fluid-limit) queueing network with \(n=3\) **classes**:

1. Class 1 â€“ interactive (chat, IDE, UI).  
2. Class 2 â€“ background (indexing, batch tools, fine-tuning).  
3. Class 3 â€“ maintenance (compaction, vacuum, WAL rotation).

Let:

- \(Q_i(t)\) = queue length of class \(i\) at time \(t\).  
- \(\lambda_i\) = average arrival rate (jobs / second).  
- \(\mu_i^{\max}\) = maximum service rate (jobs / second) when fully scheduled.  
- \(\rho_i = \lambda_i/\mu_i^{\max}\) = nominal load.  
- \(w_i > 0\) = priority weight.

We collect in vector form: \(\mathbf{Q}(t) = (Q_1(t),Q_2(t),Q_3(t))^\top\).

### 3.2 Lyapunov function and MaxWeight policy

We fix weights:
$$
w_1 = 1,\quad w_2 = 10,\quad w_3 = 100.
$$
Define a **quadratic Lyapunov function**:
$$
L(\mathbf{Q}) = Q_1^2 + 100 Q_2^2 + 10000 Q_3^2
= \mathbf{Q}^\top \operatorname{diag}(1,100,10000)\,\mathbf{Q}.
$$

At each decision epoch, we choose which class to serve by **MaxWeight**:

Let \(\mu_i(t)\) be the estimated instantaneous service rate (tokens/s) for class \(i\). Define the pressure:
$$
\Phi_i(\mathbf{Q}(t)) = w_i\,Q_i(t)\,\mu_i(t).
$$

> **Policy K1 (MaxWeight).**  
> Choose
> $$
> i^\*(t) = \arg\max_{i \in \{1,2,3\}} \Phi_i(\mathbf{Q}(t)),
> $$
> and allocate the next quantum of compute (e.g. a token generation step or batching slot) to class \(i^\*\).

This is the policy implemented in `goni-scheduler`.

### 3.3 Stability condition

We assume the **capacity region**:
$$
\mathcal{C} = \left\{ \boldsymbol{\lambda} \in \mathbb{R}_+^3 : \sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < 1 \right\}.
$$

We operationalise this with a safety factor \(\alpha \in (0,1)\), and configure admission control so that:
$$
\sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < \alpha
\quad\text{with }\alpha = 0.94\text{ by default.}
$$

> **Theorem 3.1 (Queue stability, fluid limit).**  
> Under mild assumptions on arrivals (e.g. ergodic, bounded second moments), if the arrival rate vector \(\boldsymbol{\lambda}\) lies in the interior of \(\alpha \mathcal{C}\) with \(\alpha < 1\), then the MaxWeight policy K1 stabilises the network in the sense that:
> $$
> \sup_{t \ge 0} \mathbb{E}[L(\mathbf{Q}(t))] < \infty,
> $$
> and the fluid limits of \(\mathbf{Q}(t)\) converge to 0.

> **Invariant K1 (Configured stability).**  
> The node enforces a token-budget admission control policy such that the empirical estimate of \(\boldsymbol{\lambda}\) satisfies:
> $$
> \sum_{i=1}^3 \frac{\hat{\lambda}_i}{\mu_i^{\max}} \le 0.94.
> $$
> Simulation tests must show \(\mathbb{E}[L(\mathbf{Q}(t))]\) remains bounded over long horizons under representative workloads.

### 3.4 Model router

We distinguish two model classes:

- Small model \(M_s\) with cost \(c_s\) (tokens/s, energy).  
- Large model \(M_\ell\) with cost \(c_\ell \gg c_s\).

For a request \(x\) and preliminary small-model answer \(\hat{y}_s\), we compute a **calibrated confidence** \(p(x) \in [0,1]\).

Router policy:

1. If \(p(x) > \tau_{\mathrm{accept}}\): accept small model output.  
2. If \(p(x) < \tau_{\mathrm{escalate}}\) and early in the generation: escalate.  
3. Else compare expected value of escalation vs continuation.

Escalation to the cloud-side multi-model path (the [LLM Council](/blueprint/docs/llm-council.md)) follows the triggers in Section 3 of that doc: explicit user request, high difficulty/safety-critical classification, or long-context needs that exceed local comfort.

We treat this as a **two-armed bandit** with side information (the features used to estimate \(p(x)\)).

> **Theorem 3.2 (Regret bound, sketch).**  
> Suppose the confidence estimator is \(\epsilon\)-calibrated and the reward gap between correct/incorrect decisions is bounded. Then there exists a threshold policy (approximated by our router) whose regret \(R_T\) over \(T\) requests satisfies:
> $$
> \frac{R_T}{T} \le \beta(\epsilon)
> $$
> with \(\beta(\epsilon)\) small. In practice we target \(\beta(\epsilon) \le 0.07\).

> **Invariant K2 (Router regret).**  
> On benchmark datasets, empirical regret of `goni-router` compared to an oracle policy that knows ground-truth â€œdifficultyâ€ labels must stay below 0.07.


### 3.5 App ecosystem, identity, and remote presence

We treat product completeness as part of the architecture, not a UI afterthought.
The Control and Execution planes expose explicit slots for identity, packaging,
and remote access:

- **Identity plane (logical):** user identity, agent identity, capability issuance,
  and audit attribution. This binds UI sessions to agent actions and logs.
- **Marketplace/install flow:** signed agent packages, manifest validation, policy
  prompts, and budget enforcement before activation.
- **Remote presence:** secure tunnels are modeled as capability-gated tools; there
  is no implicit "open port" path. Remote access is revocable and logged.

This section is a structural requirement derived from reference product patterns
(see `blueprint/docs/reference-products/olares.md`).


---

## 4. Execution Substrate \(\mathcal{E}\)

### 4.1 Engines and capabilities

Let \(\mathcal{M}\) be the set of models (LLMs, embedding models, classifiers). For each model \(m \in \mathcal{M}\) we define capability descriptors:

- Max context length \(L_m\).  
- Approximate throughput \(\theta_m\) (tokens/s).  
- Memory footprint \(R_m\) (RAM/VRAM).  
- Supported quantisations, devices, etc.

In code this is a struct:

```rust
pub struct Capability {
    pub max_ctx: usize,
    pub throughput_toks_per_s: f32,
    pub mem_bytes: u64,
    pub device: DeviceKind,
    // ...
}
```

The Control Plane queries these capabilities via a total function:
$$
\mathsf{cap} : \mathcal{M} \to \mathsf{Capability}.
$$

### 4.2 Encoders -> Predictor -> (Optional) Decoder (latent-first pipeline)

This repo is model-agnostic, but the execution substrate supports a common pattern:

1) **Encoders**: map one or more inputs into latent representations.  
   - Examples: text encoder, vision encoder (screenshots/images), audio encoder, structured-data encoder.  
   - Output: latent vectors + lightweight structured features.

2) **Predictor (cognitive core)**: updates latent state and selects actions.  
   - Input: (a) current latent state, (b) new latent observations, (c) an optional query/goal.  
   - Output: updated latent state, tool calls, and optionally a latent "answer" representation.

3) **Optional Decoder (verbaliser / renderer)**: turns latent state into words or other outputs.  
   - Used for: explanations, drafts, chat UX, external communications.  
   - Not required for internal planning/tool use.

See `blueprint/software/30-components/latent-predictor.md` for the integration sketch and VL-JEPA-inspired block diagram.

#### Latent-space objective (conceptual)

Where a component is trained or fine-tuned, the preferred high-level objective is:

- learn an encoder representation `S(Â·)`,
- learn a predictor `P(Â·)` such that `P(S(context), S(observation), q) â‰ˆ S(target)`,
- compare predicted vs target latent representations with a similarity loss (e.g., cosine / contrastive).

This makes "meaning" the primary internal currency, while tokens remain an interface.

#### Why this fits Goni's infra stance

- Compatible with queues/planes: encoders emit events; predictor consumes events; decoder is a late-stage consumer.  
- Compatible with local-first: always-on encoders + predictor can be small; decoder can be on-demand.  
- Compatible with multi-model arbitration: different encoders/decoders can share the same latent state contract.

### 4.3 Wasm sandboxes as effectful morphisms

We treat each **tool** or **agent** as a partial function over Arrow objects:
$$
T : S \rightsquigarrow T
$$
implemented as a Wasm module in an **effectful category** \((\mathcal{A}^\mathsf{eff})\) that extends \((\mathcal{A})\) with side-effects (timers, network, file I/O) via capabilities.

We enforce:

> **Invariant E1 (Capability safety).**
> For each sandboxed module \((W)\), there exists a declared capability set \((\mathsf{Cap}(W))\). The host ensures that any effect in \((\mathcal{A}^\mathsf{eff})\) performed by \((W)\) is an element of \((\mathsf{Cap}(W)).

This is enforced by a narrow host API surface (`goni-tool-api`), WASI-like capability handles, and resource limits.

### 4.4 Deterministic inference profile

Self-loop / agentic runs have positive Lyapunov exponents (small numeric noise can change token choice). We therefore define a **deterministic profile** for engines:

> **Invariant E2 (Deterministic preset).**
> If a request is marked deterministic, the engine must execute with:
> - temperature 0 and fixed seed (if backend supports `seed`),
> - batch size 1 and no continuous/dynamic batching,
> - single worker / single thread (or CPU-only path) and TF32 disabled on NVIDIA,
> - fixed backend flags (e.g. vLLM `--enable-deterministic-inference`), and
> - recorded blueprint/hardware/driver hashes in the log.
>
> A compliant engine may fall back to a slower profile to satisfy E2, but must not silently drop the request.

---

## 5. End-to-end semantics

At the highest level, a **request** is an element of a type:
$$
\mathsf{Req} = (\text{user_msg}, \text{tools}, \text{profile}, \text{budgets}, \dots)
$$
and a **response stream** is a sequence of tokens plus logs and tool results.

We can regard the node as computing a (possibly stochastic) function:
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$

Implementation-wise, this is the composition of:

1. Parsing / admission: \((\mathsf{Req} \to \text{job in class } i)\).
2. Data Plane retrieval: functors in \((\mathcal{A}).
3. Context selection: optimisation in \((\mathcal{X})\) with guarantee C1.
4. Scheduling + routing: \((\mathcal{K})\) with invariants K1, K2.
5. Engine execution: morphisms in \((\mathcal{E}).
6. Logging: \((\mathcal{A})\) again (metrics as Arrow tables).

Our **architectural contract** is that each piece satisfies its invariants; the composition then has predictable boundaries on resource use, latency and information quality.

---

## 6. Implementation overview (code â†” math)

| Plane / Object | Formal notion                                      | Main Rust crates                               |
| -------------- | -------------------------------------------------- | ---------------------------------------------- |
| \((\mathcal{A})\)  | \((\mathcal{A}_{rr}^{\text{affine}})\)                 | `goni-arrow`, `goni-store`, `goni-index`       |
| \((\mathcal{X})\)  | submodular optimisation over \((2^V)\)                 | `goni-context`, `goni-prompt`                  |
| \((\mathcal{K})\)  | queueing network + Lyapunov scheduler, router      | `goni-scheduler`, `goni-router`, `goni-resman` |
| \((\mathcal{E})\)  | engines and sandboxes (\((\mathcal{A}^\mathsf{eff}))\) | `goni-engine-*`, `goni-wasm`, `goni-tool-api`  |

All future contributions should be expressible as:

* new objects / morphisms in \((\mathcal{A}),
* new objective terms or constraints in \((\mathcal{X}),
* new classes or policies in \((\mathcal{K}), or
* new engines / sandboxes in \((\mathcal{E}),

without breaking the stated invariants.

