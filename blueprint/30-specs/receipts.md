---
id: REC-01
type: SPEC
status: specified_only
---
# Receipts (REC-01)
DOC-ID: REC-01

Status: Specified only / roadmap

Receipts are immutable records of mediated actions. They must be minimal by
default and verifiable via hash chaining.

Receipts are a Goni-kernel primitive. Third-party gateways, tool hosts, or
assistant frameworks may emit their own logs, but those logs do not substitute
for a canonical Goni receipt.

## Receipt tiers

Receipt volume must not drown governance evidence. Goni therefore distinguishes
receipt tier from ordinary telemetry:

| Tier | Purpose | Examples | Retention posture |
| --- | --- | --- | --- |
| `governance` | Authority, approval, policy, or irreversible action evidence. | approvals, denied actions, external side effects, model promotion, policy override | durable, hash-chained |
| `execution` | Normal mediated work evidence. | tool call, model route, memory read/write, parser-mediated ingestion | durable or compactable by policy |
| `summary` | High-volume background or batch rollups. | index rebuild summary, scheduled audit summary, Daily Brief generation | durable summary with refs to sampled spans |
| `telemetry` | Performance and health signals. | latency, queue depth, cache hit, thermal signal | metrics store; not a substitute for receipts |

The receipt tier MUST be derivable from `action_type`, `task_class`, policy
decision, risk class, and side-effect class. Governance-tier events may not be
silently downgraded to telemetry. High-volume background events may use summary
receipts if the summary preserves checked scope, inputs/outputs by hash, policy
decision, sampled span refs, and failure counts.

## PROV-DM mapping
- Entity: input/output artifacts
- Activity: toolcall, redact, retrieve, write
- Agent: user, kernel module, tool executor

## Required fields
- receipt_id
- timestamp
- trace_id
- span_id
- actor_id
- action_type
- capability_id
- task_class
- autonomy_mode
- policy_decision
- decision_basis
- risk_score
- risk_basis
- budget_delta
- input_hash
- output_hash
- memory_read_refs
- memory_diff_refs

## Delegation fields (required for delegated or tool-mediated actions)
- task_class
- autonomy_mode
- risk_score
- risk_basis
- interaction_mode
- work_order_id
- done_contract_hash
- clarification_decision
- objective_option_count
- delegation

The `delegation` object MUST expose stable delegation-engineering fields:

- `assumptions`
- `uncertainty_level`
- `question_strategy`
- `tool_intent`
- `delegation_outcome`
- `undo_strategy_ref`

## Receipt completeness profile (normative)
- `trace_id` identifies the request/run trace for correlation across spans.
- `span_id` identifies the exact operation span that emitted the receipt.
- `decision_basis` captures the basis for policy mediation (for example:
  `policy_hash`, matching rule IDs, approval references, `intent_summary`, and
  `plan_summary`).
- `memory_diff_refs` is a list of state/memory delta IDs caused by the action.
  Use an empty list when no memory mutation occurred.
- `memory_read_refs` is a list of memory IDs, chunk IDs, context item IDs, or
  retrieval result refs read by the action. Use an empty list when no memory
  was read.
- `retrieval_basis` records the retrieval mode, index refs, selected context
  refs, reranker, permission filters, and policy hash when retrieval affected
  output or execution. It must not store raw retrieved text by default.
- `bundle_id`, `manifest_hash`, and `eval_receipt_refs` record governed model
  bundle provenance when a model route depends on an approved bundle.
- `llm_route` records model-routing decisions when an LLM path is selected.
  It captures the selected route, local/Council rationale, task
  classification, models considered and used, redaction requirement, privacy
  class sent, cost/latency estimate labels, confidence label, and policy
  decision. It must not store raw prompt or retrieved text.
- `assurance_level`, `ml_bom_ref`, and `attestation_refs` record the local
  installation trust state when model bundle governance affects the route.
- `receipt_tier` records the governance/execution/summary/telemetry tier used
  for retention and review policy.
- `parser_basis` records parser identity, source hash, structure kind,
  confidence flags, chunk refs, and policy filters when parsing affected memory
  or context.
- `interaction_mode` records whether the turn was delegated execution or
  co-creation.
- `work_order_id` references the canonical pre-execution Work Order.
- `done_contract_hash` references the hashed completion contract used for the
  action.
- `clarification_decision` records the control-plane branch:
  `assume | ask_decisive | propose_objectives | block`.
- `objective_option_count` records how many candidate objectives were surfaced,
  if any.
- `delegation.assumptions` lists material assumptions surfaced to the operator
  or carried forward by policy.
- `delegation.uncertainty_level` records the confidence band used for the
  execution decision.
- `delegation.question_strategy` records whether clarification was skipped,
  asked decisively, or overridden by the user/policy.
- `delegation.tool_intent` captures the purpose of the concrete tool action,
  distinct from the user-level goal.
- `delegation.delegation_outcome` records whether the action was executed,
  queued for review, blocked, escalated, or approved.
- `delegation.undo_strategy_ref` records the rollback/compensation strategy
  reference when one exists.

## Upstream
- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)

## Downstream
- [Receipt schema](/blueprint/docs/receipts/receipt-schema.md)
- goni-prototype-lab:goni-lab/TRACEABILITY.md

## Adjacent
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [System map](/blueprint/docs/00-system-map.md)

## Conformance tests
- any mediated action must emit exactly one receipt
- receipts must form a valid hash chain
- receipts must omit raw content by default
- receipts must include `trace_id`, `span_id`, `decision_basis`, and
  `memory_read_refs` and `memory_diff_refs`
- retrieval-mediated receipts must include `retrieval_basis` when retrieved
  memory affected output or execution
- model-routing receipts must include `llm_route` when model selection affected
  output or remote escalation eligibility
- receipts must include `task_class`, `autonomy_mode`, `risk_score`, and
  `risk_basis` for any delegated action
- delegated/tool-mediated receipts must preserve `interaction_mode`,
  `work_order_id`, `done_contract_hash`, `clarification_decision`, and
  `objective_option_count`
- delegated/tool-mediated receipts must include `delegation.assumptions`,
  `delegation.uncertainty_level`, `delegation.question_strategy`,
  `delegation.tool_intent`, `delegation.delegation_outcome`, and
  `delegation.undo_strategy_ref`
- receipt `decision_basis` must preserve `intent_summary` and `plan_summary`
  when a mutating action is proposed or executed
- third-party framework logs or audit events must not be accepted as the sole
  terminal record of a mediated effect
- governance-tier events must not be downgraded to telemetry-only records
- parser-mediated memory/context changes must preserve `parser_basis`




