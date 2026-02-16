---
id: METER-01
type: SPEC
status: specified_only
---
# SPEC-METER-01 Execution Metering and Budget Units
DOC-ID: METER-01
Status: Specified only / roadmap

This spec defines runtime metering for prototype governance, QoS, and budget enforcement.

## 1. Execution taxonomy (normative)
Goni MUST classify each event into one execution layer:
- `action`: user-visible outcome (for example: prepare daily brief).
- `tool`: side-effectful tool operation (for example: send email).
- `model`: inference operation (local or remote).

## 2. Metering dimensions (normative)
For each execution, the runtime MUST capture where applicable:
- `tokens_in`, `tokens_out`
- `latency_ms`
- `tool_calls`
- `bytes_egress`
- `ram_peak_bytes`

## 3. Attribution rules (normative)
- Interactive and background work MUST be tagged so resource use is attributable.
- Shared background work (indexing/compaction) MUST be recorded separately from user actions.

## 4. Retry and failure policy (normative)
- Every attempt MUST be receipted.
- Retry policy MUST state whether resource attribution is per attempt or per successful execution.
- Automatic retries MUST keep parent execution linkage (`parent_execution_id`).

## 5. Budget enforcement
- Runtime MUST enforce hard and soft budgets at policy gate boundaries.
- Exceeding hard budgets MUST stop further remote/model execution.

## 6. Related specs
- [Receipts](/blueprint/30-specs/receipts.md)
- [Receipt metering fields](/blueprint/30-specs/metering/SPEC-METER-02-receipt-metering-fields.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## Conformance tests
- Execution records include one valid `execution_type`.
- Metering counters are non-negative and present when applicable.
- Retries preserve parent-child execution linkage.
- Hard budgets block additional remote calls.
