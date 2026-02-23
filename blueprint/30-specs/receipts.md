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

## Receipt completeness profile (normative)
- `trace_id` identifies the request/run trace for correlation across spans.
- `span_id` identifies the exact operation span that emitted the receipt.
- `decision_basis` captures the basis for policy mediation (for example:
  `policy_hash`, matching rule IDs, and approval references).
- `memory_diff_refs` is a list of state/memory delta IDs caused by the action.
  Use an empty list when no memory mutation occurred.

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




