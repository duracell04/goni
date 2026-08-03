---
id: GONI-IMAP-4AAD5EB71FAD
title: 1.3 Control Plane \(\mathcal{K}\) – Queueing & Router
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Object.** A controlled queueing network with: Three classes \(i \in \{1,2,3\}\) (interactive, background, maintenance).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 1.3 Control Plane \(\mathcal{K}\) – Queueing & Router
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 1.3 Control Plane \(\mathcal{K}\) – Queueing & Router

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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

#### 1.3.3 Delegation: autonomy and escalation quality

**Invariant K3 (risk-bounded autonomy).**

No mutating action may execute autonomously unless:

- a corridor policy is present for its `task_class`, and
- computed `risk_score` is below the active threshold for that corridor.

Define:

- Autonomous execution rate
  $$
  \mathrm{AER} = \frac{N_{\text{autonomous}}}{N_{\text{delegable}}}
  $$
- Unsafe autonomy incident rate
  $$
  \mathrm{UAI} = \frac{N_{\text{policy\_violations\_post\_commit}}}{N_{\text{autonomous}}}
  $$

Target direction:

- maximize AER for routine classes,
- keep UAI near zero with bounded escalation latency.

**Empirical check (MVP).**

- Replay labelled delegation traces and assert:
  - interaction mode is correct for recoverable vs genuinely unsettled goals,
  - all autonomous commits have corridor + threshold evidence in receipts,
  - all over-threshold actions are blocked or escalated,
  - delegated actions preserve `interaction_mode`, `work_order_id`, and
    `done_contract_hash`,
  - all mutating delegated actions preserve `intent_summary`,
    `plan_summary`, and `tool_intent`,
  - actions taken under ambiguity include surfaced assumptions and a
    clarification strategy in receipts.
- Evaluate escalation quality on labelled events:
  - escalation precision/recall for high-risk actions,
  - rollback/compensation success for mistaken autonomous actions,
  - clarification efficiency and question value on vague-intent traces,
  - branch-efficiency under fixed token budgets.

**Schema/evidence alignment check (MVP).**

- Receipt schema, receipt spec, and AUTON eval metrics must agree on the
  presence and naming of:
  - `task_class`
  - `autonomy_mode`
  - `risk_score`
  - `risk_basis`
  - `delegation.assumptions`
  - `delegation.uncertainty_level`
  - `delegation.question_strategy`
  - `delegation.tool_intent`
  - `delegation.delegation_outcome`

---
