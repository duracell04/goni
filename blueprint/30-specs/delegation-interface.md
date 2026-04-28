---
id: DELEG-INT-01
type: SPEC
status: specified_only
---
# DELEG-INT-01 - Delegation Interface
DOC-ID: DELEG-INT-01
Status: Specified only / roadmap

This spec defines the pre-execution control plane that reconstructs user intent
into kernel-governed execution objects before corridor policy, tool mediation,
or receipt emission occurs.

## 1. Purpose

Goni must not treat every user message as a literal execution prompt. Before
execution, the runtime MUST:

- classify whether the task is delegated execution or co-creation,
- define what "done" means for the current turn,
- decide whether to assume, ask, propose candidate objectives, or block,
- decide whether the task requires audit-grade work,
- compile a stable Work Order that downstream policy and tools can audit.

This interface exists so prompt-work and audit-work become inspectable
control-plane state rather than hidden model behavior.

## 2. Normative objects

### 2.1 InteractionMode

`InteractionMode = delegation | co_creation`

- `delegation`: the user objective is recoverable from context, policy, or
  stable defaults.
- `co_creation`: the objective itself is materially ambiguous and must be
  narrowed before execution.

### 2.2 ClarificationDecision

`ClarificationDecision = assume | ask_decisive | propose_objectives | block`

- `assume`: proceed under surfaced assumptions.
- `ask_decisive`: ask one bounded question whose answer materially changes the
  outcome.
- `propose_objectives`: surface at most two candidate objectives and require
  user selection.
- `block`: refuse execution because safe progress is not possible.

### 2.3 WorkQualityMode

`WorkQualityMode = best_effort | audit_grade`

- `best_effort`: bounded, reversible, low-stakes work where an incomplete search
  may be acceptable if assumptions are surfaced.
- `audit_grade`: conservative work mode for verification, compliance,
  contribution review, security, negative claims, or any task where a missing
  source could make the conclusion misleading.

Audit-grade is about the **work Goni performs**, not merely the final answer
style. It changes search breadth, evidence handling, claim strength, receipts,
and conformance expectations.

### 2.4 ClaimStrength

`ClaimStrength = proved | supported | unknown | disproved`

- `proved`: complete-enough scope plus direct evidence.
- `supported`: strong evidence, but not exhaustive.
- `unknown`: scope incomplete or decisive evidence missing.
- `disproved`: complete-enough scope plus contrary evidence.

### 2.5 DoneContract

Every executable turn MUST have a `DoneContract` with:

- `deliverable`
- `must_include`
- `must_verify`
- `stop_condition`

The Done Contract is the kernel-visible statement of what counts as finished.
It must be hashable, stable across retries, and compact enough to reference in
receipts and audit records.

For `audit_grade` work, the Done Contract MUST also identify:

- the minimum evidence scope required for the conclusion,
- the allowed strength of negative claims,
- and whether missing evidence must be surfaced before completion.

### 2.6 WorkOrder

Every executable turn MUST compile a `WorkOrder` with:

- `goal`
- `done_contract`
- `inputs`
- `constraints`
- `assumptions`
- `plan`
- `tools`
- `risk_class`
- `output_schema`
- `work_quality_mode`

For `audit_grade` work, the Work Order MUST additionally carry:

- `evidence_scope`: sources, refs, paths, time windows, artifacts, and explicit
  exclusions.
- `search_strategy`: the planned coverage pattern, including branches, repos,
  PRs/issues, logs, local/remote deltas, or other relevant surfaces.
- `negative_claim_policy`: how absence-of-evidence claims may be phrased.
- `claim_strength_target`: the strongest claim the current scope can support.
- `missing_evidence_plan`: what remains unchecked and what would close the
  loop.
- `audit_sticky`: whether audit-grade mode persists across follow-up turns.

The Work Order is the canonical pre-execution object. Downstream components may
store summarized or referenced forms, but the logical object MUST preserve all
of the fields above.

### 2.7 ReconstructionPreview

For user-facing previews, approvals, or operator inspection, the runtime MUST
be able to produce a `ReconstructionPreview` containing:

- `goal`
- `done`
- `assumptions`
- `risk`
- `question`

For `audit_grade` work, the preview SHOULD also expose:

- `scope_checked_or_planned`
- `missing_evidence`
- `claim_strength_limit`

The preview is derived from the Work Order and policy state. It must not rely
on UI-only shadow state.

## 3. Pre-execution pipeline

Before any mutating tool proposal, externally visible action, or audit-grade
conclusion, the runtime MUST execute the following logical pipeline:

1. `parse_intent`
2. `classify_interaction_mode`
3. `classify_work_quality_mode`
4. `decide ask / assume / propose / block`
5. `compile_work_order`
6. `execute_under_corridor_policy`
7. `emit_receipt`

The pipeline is ordered. A later stage may not silently redefine the goal,
Done Contract, scope, or claim-strength boundary without producing a new Work
Order reference and updated receipt metadata.

## 4. Decision rules

### 4.1 Delegation vs co-creation classification

The runtime MUST choose `delegation` when:

- the objective is recoverable from active policy, prior context, or stable
  task defaults, and
- the unresolved variables concern execution detail rather than goal identity.

The runtime MUST choose `co_creation` when:

- multiple materially different objectives remain plausible, and
- silently selecting one would define the user's goal rather than execute it.

Co-creation is about unresolved objectives, not missing factual details. If the
goal is known and only a factual parameter is missing, the runtime should stay
in `delegation` and choose among `assume`, `ask_decisive`, or `block`.

### 4.2 Work-quality classification

The runtime MUST choose `audit_grade` when any of the following hold:

- the user asks for audit, verification, compliance, contribution review,
  status proof, or absence/presence of evidence,
- the task requires a negative claim such as "not present", "no contribution",
  "nothing changed", or "does not exist",
- the output can affect legal, financial, security, governance, publishing, or
  irreversible operational decisions,
- the task depends on coverage across branches, repos, logs, PRs, issues,
  attachments, or build artifacts.

The runtime MAY choose `best_effort` only when the work is low-stakes,
reversible, and the user explicitly or implicitly accepts bounded exploration.

### 4.3 Clarification decision function

The runtime MUST choose:

- `assume` when the task is reversible or low-risk and surfaced assumptions are
  sufficient,
- `ask_decisive` when one answer materially changes plan, risk, tool choice,
  irreversibility, or audit scope,
- `propose_objectives` only in `co_creation` mode,
- `block` when safe execution is impossible under current policy and context.

The runtime MUST NOT ask questions that can be answered from:

- active policy,
- retrieved context,
- prior approvals,
- deterministic task constraints,
- or stable user defaults.

### 4.4 Audit-grade work rules

For `audit_grade` work, the runtime MUST follow these epistemic rules:

- **Absence-of-evidence rule:** absence of evidence in scope `S` is not evidence
  of absence outside scope `S`.
- **Scope declaration:** the Work Order must declare the planned or checked
  scope before strong conclusions are made.
- **Evidence before inference:** observed artifacts and derived conclusions must
  remain separable in receipts and user-facing summaries.
- **Negative-claim burden:** negative claims require stronger coverage than
  positive claims.
- **Missing-evidence surfacing:** if the scope is incomplete, the runtime must
  preserve what is missing and what next check would close the loop.
- **Sticky audit mode:** audit-grade mode persists for follow-up turns in the
  same task/session unless explicitly reset or a clear unrelated task boundary
  is detected and surfaced.

## 5. Storage and reference contract

The control plane MUST preserve stable references for the pre-execution object:

- `work_order_id`
- `interaction_mode`
- `work_quality_mode`
- `done_contract_hash`
- `clarification_decision`
- `objective_option_count`

For `audit_grade` work, it MUST also preserve:

- `evidence_scope_ref`
- `search_strategy_ref`
- `claim_strength`
- `missing_evidence_ref`
- `audit_sticky`

Mutating actions with a compensation path SHOULD also preserve:

- `undo_strategy_ref`

These references may be stored in summarized form, but they must remain
replayable and auditable across tool calls, receipts, and review flows.

## 6. Defaults and limits

Unless stricter policy overrides them, the runtime defaults are:

- at most one decisive question per task,
- at most two objective options when `clarification_decision =
  propose_objectives`,
- one target output before optional transformations,
- explicit surfaced assumptions whenever execution proceeds without asking,
- audit-grade mode is sticky inside the same task/session until reset.

## 7. Invariants

- **I1 - Work-order first:** mutating execution and audit-grade conclusions
  require a Work Order.
- **I2 - Done-contract completeness:** executable turns require a Done Contract.
- **I3 - No silent goal selection:** unresolved objective ambiguity must route
  to `co_creation` or `block`, not hidden defaulting.
- **I4 - One-question bound:** decisive clarification is bounded and
  policy-controlled.
- **I5 - Kernel-backed preview:** previews and approval panels must be
  derivable from kernel state.
- **I6 - Stable references:** receipts and audit records must preserve
  `interaction_mode`, `work_quality_mode`, `work_order_id`, and
  `done_contract_hash` for delegated actions.
- **I7 - Audit-grade work:** verification and compliance tasks must preserve
  scope, evidence, inference, missing-evidence, and claim-strength metadata.
- **I8 - No absence laundering:** the runtime must not turn "not found in checked
  scope" into "does not exist".

## 8. Related specs

- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 9. Upstream

- [Product vision](/blueprint/10-product/10-vision.md)
- [Delegation doctrine](/blueprint/10-product/15-delegation-doctrine.md)

## 10. Downstream

- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [API surface](/blueprint/software/40-apis-and-ui/api-surface.md)
- [Dashboard concepts](/blueprint/software/40-apis-and-ui/dashboard-concepts.md)

## 11. Adjacent

- [Metrics](/blueprint/docs/metrics.md)
- [Eval lane](/blueprint/50-evidence/eval/README.md)

## Conformance tests

- turns with recoverable objectives must classify as `delegation`
- turns with genuine objective ambiguity must classify as `co_creation`
- audit, compliance, contribution-review, verification, and negative-claim
  tasks must classify as `audit_grade`
- `ask_decisive` must occur only when one answer materially changes plan, risk,
  tool choice, irreversibility, or audit scope
- `propose_objectives` must never emit more than two options by default
- execution that proceeds under `assume` must surface assumptions in receipts
- preview data must be reproducible from Work Order + policy state
- audit-grade Work Orders must include evidence scope, search strategy, missing
  evidence, and claim-strength metadata
- negative claims must be downgraded to "not found in checked scope" unless the
  Work Order proves adequate coverage
- sticky audit mode must persist across follow-up turns unless reset or a clear
  unrelated task boundary is surfaced
