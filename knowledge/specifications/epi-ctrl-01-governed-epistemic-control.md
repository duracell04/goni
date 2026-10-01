---
id: EPI-CTRL-01
title: Governed Epistemic Control
type: specification
status: draft
implementation_state: specified_only
proposition: For evidence-sensitive Work Orders, GONI may maintain a bounded transient epistemic state over competing hypotheses, evidence, evidence lineage, contradictions, and unresolved evidence needs, and use that state to govern further retrieval, support revision, and the decision to continue, commit, or surface unresolved uncertainty.
domains:
- agent
- kernel
- memory
- research
- software
- system
aliases:
- EPISTEMIC-CONTROLLER
- EPISTEMIC-FRAME
relations:
- type: depends_on
  target: EPI-SEP-01
- type: depends_on
  target: MEM-RETR-01
- type: depends_on
  target: CTX-COMP-01
- type: depends_on
  target: CTX-MISS-01
- type: depends_on
  target: ITCR-01
- type: depends_on
  target: GONI-SPEC-59E3DFC23CC8
- type: depends_on
  target: GONI-SPEC-914D01705CED
- type: depends_on
  target: GONI-SPEC-A742123055E0
sources:
- SRC-LINDLEY1956-EXPERIMENT-INFORMATION
- SRC-SETTLES2009-ACTIVE-LEARNING
- SRC-JIANG2023-FLARE
- SRC-ASAI2024-SELF-RAG
- SRC-GAO2023-RARR
artifacts: []
uncertainty: This contract specifies a proposed epistemic-control protocol, not an implemented or calibrated truth engine. Hypothesis quality, evidence-lineage inference, probability calibration, query-value estimation, stopping thresholds, latency, cost, and downstream utility require matched GONI evaluation.
legacy: []
---

# Governed Epistemic Control

> Status boundary: this is a specified-only architectural contract. Normative
> language describes intended conformance behavior rather than observed
> implementation, verification, calibration, or non-bypassability.

## 1. Scope

EPI-CTRL-01 governs evidence-sensitive cognition in which a Work Order may
require the system to distinguish between plausible explanations, acquire
additional evidence, resolve contradictions, or decide that available evidence
is insufficient.

It does not create a new architectural plane, a new durable memory database, or
a new source of authority.

The compact control loop is:

> **Observe -> Hypothesize -> Discriminate -> Retrieve -> Update -> Commit / Continue / Surface uncertainty**

The controller coordinates existing Context and Control Plane responsibilities.
Durable state remains governed by existing MemoryEntries and source-of-truth
contracts.

## 2. EpistemicFrame

For a Work Order at step \(t\), the transient epistemic state may be represented
conceptually as:

\[
\mathcal{E}_t =
(\mathcal{H}_t, \mathcal{D}_t, \mathcal{L}_t, \mathcal{C}_t, \mathcal{Q}_t, B_t),
\]

where:

- \(\mathcal{H}_t\) is the set of active candidate hypotheses;
- \(\mathcal{D}_t\) is the evidence set available to the Work Order;
- \(\mathcal{L}_t\) is evidence and derivation lineage;
- \(\mathcal{C}_t\) is the set of unresolved contradictions;
- \(\mathcal{Q}_t\) is the set of unresolved evidence questions or candidate tests;
- \(B_t\) is the remaining bounded research and reasoning budget.

A logical `EpistemicFrame` SHOULD preserve, where applicable:

```yaml
epistemic_frame_id:
work_order_id:
context_pack_id:
inference_frame_id:
hypotheses:
  - hypothesis_id:
    proposition:
    claim_strength:
    parent_hypothesis_refs:
    supporting_evidence_refs:
    contradicting_evidence_refs:
    evidence_lineage_refs:
    unresolved_requirements:
contradictions: []
evidence_gaps: []
candidate_tests: []
research_budget:
selection_policy:
stop_state:
created_at:
provenance:
receipt_ref:
```

The frame records externally reviewable epistemic state. It MUST NOT require raw
private chain-of-thought disclosure.

## 3. Persistence boundary

`EpistemicFrame` is a transient Work-Order-bound control representation, not a
parallel canonical memory store.

If an unresolved proposition becomes durable, it MUST use the existing
MemoryEntries lifecycle and remain `hypothesis` or `derived` unless the
applicable evidence-backed promotion contract is satisfied.

The controller MUST NOT create an `EpistemicStore`, `HypothesisStore`, or other
parallel source of truth merely to preserve reasoning state.

## 4. Hypothesis set

For evidence-sensitive work, cognition MAY maintain multiple candidate
hypotheses rather than collapsing immediately onto the first plausible
interpretation.

A conforming profile SHOULD preserve:

- at least one explicit leading hypothesis when a conclusion is being evaluated;
- materially plausible alternatives when available;
- evidence that supports or contradicts each active hypothesis;
- unresolved requirements that would materially affect commitment;
- parent or derivation references for generated alternatives.

The number of hypotheses is resource-bounded. More branches are not inherently
better.

## 5. Evidence and lineage

EPI-SEP-01 applies in full.

For proposition \(h\), let \(E(h)\) be presented supporting evidence and let
\(r(e)\) identify an independent provenance root or evidence-lineage class.

The controller distinguishes:

\[
N_{raw}(h)=|E(h)|
\]

from:

\[
N_{ind}(h)=|\{r(e): e\in E(h)\}|.
\]

Agreement among multiple artifacts MUST NOT be treated as independent
corroboration when they materially descend from the same evidence lineage.

The concrete lineage function \(r(e)\) is implementation-dependent and MAY
remain unresolved when independence cannot be established reliably.

## 6. Support revision

The default contract reuses GONI's existing `ClaimStrength` vocabulary:

`proved | supported | unknown | disproved`.

A conceptual update is:

\[
S_{t+1}(h) =
\operatorname{Update}
(S_t(h), e_{t+1}, L(e_{t+1}), C_t),
\]

where support revision accounts for the new evidence, its lineage, and
contradictions.

The exact update function is non-normative.

A probabilistic profile MAY use Bayesian or other calibrated updates, for
example:

\[
P(h\mid e) \propto P(e\mid h)P(h),
\]

only when the probabilities, likelihood model, and calibration are supported by
named evaluation evidence. Model-generated numerical confidence MUST NOT be
silently treated as a calibrated posterior.

## 7. Discriminating evidence acquisition

A context miss identifies missing material. EPI-CTRL-01 adds the stronger
research question: which permitted observation, retrieval, or clarification
would most efficiently distinguish the active hypotheses or satisfy the
Work Order's evidence requirements?

For a calibrated probabilistic profile, a conceptual expected-information-gain
objective is:

\[
\operatorname{EIG}(q)
=
H(\mathcal{H}_t)
-
\mathbb{E}_{a\sim p(a\mid q)}
[H(\mathcal{H}_{t+1}\mid a)].
\]

A cost-aware conceptual policy is:

\[
q_t^*
=
\arg\max_q
[
\operatorname{EIG}(q)
-
\lambda_c C(q)
-
\lambda_l L(q)
-
\lambda_e E(q)
-
\lambda_p R(q)
],
\]

where the penalty terms may represent compute or monetary cost, latency,
energy/resource cost, and privacy or external-dependency risk.

These equations define a design objective, not a required estimator. An
implementation MAY use heuristics, ordinal discriminative value, calibrated
models, or other policies if evaluation demonstrates acceptable quality and
cost.

## 8. Decision-aware value of information

When the Work Order exists to support a consequential decision rather than to
maximize descriptive certainty, the controller MAY use a value-of-information
objective.

Conceptually:

\[
\operatorname{VOI}(q)
=
\mathbb{E}_{a}
[
\max_d EU(d\mid \mathcal{E}_t,a)
]
-
\max_d EU(d\mid \mathcal{E}_t)
-
C(q).
\]

The controller SHOULD prefer additional inquiry when its expected decision
benefit justifies its full acquisition cost under policy and budget.

No implementation may claim calibrated expected utility or VOI without named
evaluation evidence.

## 9. ContextMiss integration

When an evidence gap requires additional material, EPI-CTRL-01 MUST re-enter
ordinary governed retrieval through CTX-MISS-01 rather than granting the model
ambient search or connector authority.

The epistemic controller may supply bounded metadata such as:

- active hypothesis references;
- the evidence gap being resolved;
- the candidate test or discriminating purpose;
- the query-selection policy;
- remaining epistemic budget.

Retrieval output returns as candidate evidence through MEM-RETR-01 and
CTX-COMP-01 before cognition resumes.

## 10. Stopping and commitment

EPI-CTRL-01 reuses the Work Order's existing DoneContract rather than defining a
second completion contract.

Let \(D_t\) denote satisfaction of the relevant DoneContract, let \(S_t(h)\)
denote the support state for the leading hypothesis, and let
\(\mathcal{C}^{critical}_t\) denote unresolved contradictions that the Work
Order requires to be resolved.

A conceptual commitment condition is:

\[
\operatorname{Commit}(h^*)
\iff
D_t
\land
S_t(h^*) \succeq \tau_W
\land
\mathcal{C}^{critical}_t = \varnothing.
\]

Further inquiry is conceptually justified while:

\[
\neg D_t
\land
B_t > 0
\land
\max_q \operatorname{Value}(q) > 0.
\]

`Value(q)` may be a validated information-gain, decision-value, or bounded
heuristic policy.

The controller terminates with one of three logical states:

- `commit` — the DoneContract permits the supported conclusion;
- `continue` — additional permitted inquiry has sufficient expected value;
- `surface_uncertainty` — the conclusion remains unresolved and further inquiry
  is unavailable, disallowed, exhausted, or not worth its full cost.

The system MUST NOT convert budget exhaustion or retrieval failure into
fabricated certainty.

## 11. Authority boundary

Epistemic confidence does not create execution authority.

\[
\boxed{
\operatorname{EpistemicSupport}
\not\Rightarrow
\operatorname{AuthorityToAct}
}
\]

Any consequential action after epistemic commitment still requires the ordinary
GONI path through mandate, policy, capability, approval, budgets, revocation,
tool mediation, and receipts.

## 12. Resource and recursion limits

The controller MUST be bounded by the applicable Work Order, ITCR, scheduler,
privacy, connector, token, latency, compute, energy, and monetary budgets.

The system MUST define finite branch, retrieval, or recursion limits for
profiles that perform iterative inquiry.

A higher hypothesis count, larger retrieval volume, or longer reasoning trace is
not a success metric by itself.

## 13. Reviewability

For audit-grade work, receipts SHOULD make it possible to reconstruct:

- which hypotheses were materially active;
- which evidence supported or contradicted them;
- which evidence items shared a lineage where known;
- which evidence gap triggered additional retrieval;
- why the controller continued, stopped, or surfaced uncertainty;
- which DoneContract threshold permitted commitment;
- which budgets constrained the research path.

Reviewability does not require storage of private chain-of-thought.

## 14. Conformance invariants

A conforming implementation must preserve at least:

- synthetic cognition counted as independent evidence: zero;
- authority expansion caused solely by epistemic support: zero;
- unbounded epistemic recursion: zero;
- unresolved critical contradictions silently omitted in audit-grade commitment:
  zero;
- budget exhaustion silently converted to certainty: zero.

Utility, calibration, query quality, latency, cost, energy, and abstention
quality are measured properties rather than assumed guarantees.
