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
- actor_id
- action_type
- capability_id
- policy_decision
- budget_delta
- input_hash
- output_hash

## Upstream
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)

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




