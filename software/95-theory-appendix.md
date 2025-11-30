# 95 – Theoretical Appendix

Status: v0.1 (2025-11-30)
Purpose: Summarise the mathematical structures behind Goni’s design.
Audience: Reviewers, academic readers, formal methods / queueing / optimisation people.

---

## 1. Category-theoretic view of the Data Plane

### 1.1 Objects: schemas, instances, and `RecordBatch`

We model the Data Plane as a symmetric monoidal category
\[
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}},
\]
whose objects are Arrow schemas and whose morphisms are affine, zero-copy transforms.

- Each schema \(S\) corresponds to an object.  
- Instances \(B_S\) correspond to generalised “states” of that object.  
- A transform \(f : S \to T\) is a morphism \(f^\#: \mathsf{Inst}(S) \to \mathsf{Inst}(T)\) respecting the affine/zero-copy constraints.

The monoidal product \(\oplus\) is **schema concatenation**, corresponding physically to `RecordBatch::try_new` with multiple columns sharing underlying buffers.

### 1.2 Relation to FinVect and FinRel

If we ignore nullability and treat fixed-length numeric arrays as vectors over a field \(k\), a `RecordBatch` can be abstracted as an element of a finite-dimensional vector space \(V\), placing us in \(\mathrm{FinVect}_k\).

However, dataflow with constraints, joins, and feedback is more naturally modelled in the category of linear relations \(\mathrm{FinRel}_k\):

- Morphisms are subspaces \(R \subseteq U \times V\), not just linear maps.  
- This allows multi-valued and partially defined transforms.  
- Caps and cups (units and counits) model feedback loops and trace operators.

Goni uses this relational perspective to reason about:

- Joins and filters as relations.  
- Feedback between data and scheduler decisions.  
- The possibility of “wiring diagrams” à la signal-flow calculus.

A design requirement is that every relational diagram used at this level must be **realizable** by an implementation in \(\mathcal{A}_{rr}^{\text{affine}}\) plus a finite set of explicit materialisation points.

---

## 2. Submodular optimisation in the Context Plane

### 2.1 Facility-location with relevance

Context selection is framed as a submodular maximisation problem:

Given:

- Ground set \(V\) (retrieved chunks).  
- Similarity kernel \(k(i,j) = \cos(e_i, e_j) \ge 0\).  
- Relevance weights \(r_j \ge 0\).  
- Costs \(c_j\) and budget \(B\).

Define:
\[
F(S) = \sum_{i \in V} \max_{j \in S} k(i,j) + \gamma \sum_{j \in S} r_j, \quad S \subseteq V.
\]

This is a classic **facility-location** term (coverage of \(V\) by facilities \(S\)) plus a modular term (relevance of facilities).

### 2.2 Monotone submodularity and greedy guarantees

It is known that:

- Each term \(F_i(S) = \max_{j\in S} k(i,j)\) is monotone submodular as a function of \(S\).  
- A non-negative sum of submodular functions is submodular.  
- Adding a non-negative modular term preserves submodularity and monotonicity.

Thus, Goni’s context objective \(F\) is **monotone submodular**. By Nemhauser et al., greedy maximisation under a cardinality or simple knapsack constraint yields:
\[
F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^*),
\]
where \(S^*\) is the optimal set. This gives a clean **approximation guarantee** for each context selection decision.

---

## 3. Queueing and Lyapunov stability in the Control Plane

### 3.1 Workload model

We consider a discrete-time queueing model with three classes:

- Interactive, background, maintenance.  
- Arrivals \(A_i(t)\) with rates \(\lambda_i\), services with maximum rates \(\mu_i^{\max}\).  
- Queue lengths \(Q_i(t)\) evolving according to:
  \[
  Q_i(t+1) = \max(Q_i(t) - S_i(t), 0) + A_i(t),
  \]
  where \(S_i(t)\) is the service given to class \(i\) at time \(t\).

The **capacity region** is:
\[
\mathcal{C} = \left\{ \boldsymbol{\lambda}\in\mathbb{R}^3_+ :
\sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < 1 \right\}.
\]

### 3.2 MaxWeight and Lyapunov drift

We define a quadratic Lyapunov function:
\[
L(\mathbf{Q}) = \mathbf{Q}^\top D\,\mathbf{Q},\quad
D = \operatorname{diag}(1, 100, 10000),
\]
assigning different penalties to different classes.

The **MaxWeight** policy chooses at each time:
\[
i^*(t) = \arg\max_i \, w_i\,Q_i(t)\,\mu_i(t),
\]
with weights \(w_i\) aligned with the diagonal of \(D\).

Standard results in stochastic network theory show that, under mild conditions (e.g. i.i.d. arrivals, bounded increments), MaxWeight stabilises any arrival vector in the interior of \(\mathcal{C}\):

- The expected one-step Lyapunov drift is negative outside a compact set.  
- This implies positive recurrence and finite expected queue lengths.

Goni leverages this to claim: **if we configure utilisation below the boundary of \(\mathcal{C}\)** (with a safety margin \(\alpha < 1\)), and use a MaxWeight-like scheduler, then interactive queues remain stable even under mixed workloads.

---

## 4. Routing as a contextual bandit

### 4.1 Setup

The model router faces a two-armed decision at each request:

- Action \(a_s\): answer with small model only.  
- Action \(a_\ell\): escalate to large model.

We assume that:

- There is an underlying random variable \(Z\) (task difficulty) that determines which model is “good enough”.  
- A learned confidence predictor \(p(x)\) estimates the probability that the small model is sufficient given features of the request and preliminary small-model output.

This is a classic **contextual bandit** problem: features are the context, the two actions yield different rewards (quality vs cost), and the router’s goal is to minimise regret vs an oracle with knowledge of \(Z\).

### 4.2 Threshold policies and regret

Under assumptions:

- The confidence predictor is \(\epsilon\)-calibrated, i.e. predicted probabilities match empirical frequencies up to error \(\epsilon\).  
- Reward gaps between good and bad actions are bounded.

One can show that simple **threshold policies** on \(p(x)\) (accept vs escalate) can achieve **bounded average regret**, with the bound depending on calibration error and reward gaps.

Goni does not attempt to derive tight theoretical bounds at this stage; rather, it:

- Formalises regret \(R_T\) and its normalisation \(R_T/T\) as key metrics.  
- Specifies a target average regret (e.g. ≤ 0.07) as a **design constraint**.  
- Validates policies empirically on labelled datasets.

This makes “small-then-big” routing a **controlled approximation**, not a hand-wavy optimisation.

---

## 5. Capability-based execution and local-first semantics

The Execution Plane is modelled as an **effectful extension** of the Data Plane:

- Pure data transforms still live in \(\mathcal{A}\).  
- Side effects (file I/O, network, time) are wrapped in capabilities and live in \(\mathcal{A}^\mathsf{eff}\).

Each Wasm tool or engine is parameterised by a **capability set** \(\mathsf{Cap}(W)\), and the host enforces:
\[
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W).
\]

The **local-first** requirement is then:

> For the core request→response function \(\mathsf{Run}\), there exists an implementation whose effect trace contains no network capabilities.

This allows us to reason about privacy and sovereignty at the level of the effect system: a conformant local node is simply one where \(\mathsf{Run}\) lives in the **sub-category of local-only effects**.

---

## 6. Summary

Goni’s architectural choices are not just “good engineering taste”; they are anchored in:

- **Category theory** for composable, zero-copy dataflow.  
- **Submodular optimisation** for context selection with approximation guarantees.  
- **Queueing theory and Lyapunov methods** for scheduler stability.  
- **Bandit theory** for model routing with bounded regret.  
- **Capability systems** for safety and local-first operation.

The conformance criteria in §30 are simply the *operationalisation* of these mathematical commitments: they specify what must be proved, what must be tested, and what it means for an implementation to “realise” the theoretical model.
