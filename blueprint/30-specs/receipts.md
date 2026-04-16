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
- memory_diff_refs

## Delegation fields (required for delegated or tool-mediated actions)
- task_class
- autonomy_mode
- risk_score
- risk_basis
- delegation

The `delegation` object MUST expose stable delegation-engineering fields:

- `assumptions`
- `uncertainty_level`
- `question_strategy`
- `tool_intent`
- `delegation_outcome`

## Receipt completeness profile (normative)
- `trace_id` identifies the request/run trace for correlation across spans.
- `span_id` identifies the exact operation span that emitted the receipt.
- `decision_basis` captures the basis for policy mediation (for example:
  `policy_hash`, matching rule IDs, approval references, `intent_summary`, and
  `plan_summary`).
- `memory_diff_refs` is a list of state/memory delta IDs caused by the action.
  Use an empty list when no memory mutation occurred.
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

## Upstream
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
  `memory_diff_refs`
- receipts must include `task_class`, `autonomy_mode`, `risk_score`, and
  `risk_basis` for any delegated action
- delegated/tool-mediated receipts must include `delegation.assumptions`,
  `delegation.uncertainty_level`, `delegation.question_strategy`,
  `delegation.tool_intent`, and `delegation.delegation_outcome`
- receipt `decision_basis` must preserve `intent_summary` and `plan_summary`
  when a mutating action is proposed or executed




