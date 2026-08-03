---
id: METER-02
type: SPEC
status: specified_only
---
# SPEC-METER-02 Receipt Metering Fields
DOC-ID: METER-02
Status: Specified only / roadmap

This spec defines minimum metering and budget fields in receipts for prototype observability and governance.

## 1. Scope
Applies to receipts emitted from `action`, `tool`, or `model` executions.

## 2. Required execution fields
Receipts for metered executions MUST include:
- `execution_id`
- `execution_type` (`action|tool|model`)
- `metering` object with relevant counters
- `budget_spent` object (budget unit -> amount)

## 3. Optional provider context
Receipts MAY include:
- `provider`
- `model_id`

## 4. Privacy constraints
- Receipts MUST avoid raw transcript storage by default.
- Metering fields MUST be computable from counters and hashes, not raw content.

## 5. Compatibility
- This spec extends [REC-01](/blueprint/30-specs/receipts.md) and aligns with `schemas/receipts/receipt.schema.json`.
- Existing non-metered receipts remain valid without optional fields.

## Conformance tests
- Metered executions emit receipts containing required execution fields.
- `execution_type` values outside `action|tool|model` are rejected.
- Budget counters are non-negative and policy-checkable.
