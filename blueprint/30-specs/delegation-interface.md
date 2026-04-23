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
- compile a stable Work Order that downstream policy and tools can audit.

This interface exists so prompt-work becomes inspectable control-plane state
rather than hidden model behavior.

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

### 2.3 DoneContract

Every executable turn MUST have a `DoneContract` with:

- `deliverable`
- `must_include`
- `must_verify`
- `stop_condition`

The Done Contract is the kernel-visible statement of what counts as finished.
It must be hashable, stable across retries, and compact enough to reference in
receipts and audit records.

### 2.4 WorkOrder

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

The Work Order is the canonical pre-execution object. Downstream components may
store summarized or referenced forms, but the logical object MUST preserve all
of the fields above.

### 2.5 ReconstructionPreview

For user-facing previews, approvals, or operator inspection, the runtime MUST
be able to produce a `ReconstructionPreview` containing:

- `goal`
- `done`
- `assumptions`
- `risk`
- `question`

The preview is derived from the Work Order and policy state. It must not rely
on UI-only shadow state.

## 3. Pre-execution pipeline

Before any mutating tool proposal or externally visible action, the runtime
MUST execute the following logical pipeline:

1. `parse_intent`
2. `classify_interaction_mode`
3. `decide ask / assume / propose / block`
4. `compile_work_order`
5. `execute_under_corridor_policy`
6. `emit_receipt`

The pipeline is ordered. A later stage may not silently redefine the goal or
Done Contract without producing a new Work Order reference and updated receipt
metadata.

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

### 4.2 Clarification decision function

The runtime MUST choose:

- `assume` when the task is reversible or low-risk and surfaced assumptions are
  sufficient,
- `ask_decisive` when one answer materially changes plan, risk, tool choice,
  or irreversibility,
- `propose_objectives` only in `co_creation` mode,
- `block` when safe execution is impossible under current policy and context.

The runtime MUST NOT ask questions that can be answered from:

- active policy,
- retrieved context,
- prior approvals,
- deterministic task constraints,
- or stable user defaults.

## 5. Storage and reference contract

The control plane MUST preserve stable references for the pre-execution object:

- `work_order_id`
- `interaction_mode`
- `done_contract_hash`
- `clarification_decision`
- `objective_option_count`

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
- explicit surfaced assumptions whenever execution proceeds without asking.

## 7. Invariants

- **I1 - Work-order first:** mutating execution requires a Work Order.
- **I2 - Done-contract completeness:** executable turns require a Done Contract.
- **I3 - No silent goal selection:** unresolved objective ambiguity must route
  to `co_creation` or `block`, not hidden defaulting.
- **I4 - One-question bound:** decisive clarification is bounded and
  policy-controlled.
- **I5 - Kernel-backed preview:** previews and approval panels must be
  derivable from kernel state.
- **I6 - Stable references:** receipts and audit records must preserve
  `interaction_mode`, `work_order_id`, and `done_contract_hash` for delegated
  actions.

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
- `ask_decisive` must occur only when one answer materially changes plan, risk,
  tool choice, or irreversibility
- `propose_objectives` must never emit more than two options by default
- execution that proceeds under `assume` must surface assumptions in receipts
- preview data must be reproducible from Work Order + policy state
