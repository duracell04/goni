---
id: SPEC-TXN-01
type: SPEC
status: specified_only
---
# SPEC-TXN-01 - Transactional Tool Execution
DOC-ID: SPEC-TXN-01
Status: Specified only / roadmap

This spec defines transactional semantics for mediated tool actions, including
atomicity boundaries, idempotency, retries, and compensation for irreversible
operations.

## 1. Transaction model

Each mutating tool operation executes in a transaction lifecycle:
`prepared -> executing -> committed | rolled_back | compensating | failed`.

`failed` indicates operator attention is required and compensation did not fully
restore intended postconditions.

## 2. Atomicity boundary

The kernel guarantees atomicity only within mediated local commit scopes:
- state deltas written through kernel-controlled stores,
- receipt append operations.

External side effects (third-party APIs, email delivery, payment rails) are
not guaranteed reversible and require compensation policy.

## 3. Idempotency rules

For any non-read-only operation:
- `idempotency_key` is mandatory,
- replay of identical tuple
  (`action`, `args_hash`, `capability_id`, `idempotency_key`) MUST return the
  original outcome,
- replay with mismatched tuple MUST be rejected and logged.

## 4. Commit protocol

### 4.1 Prepare
- validate capability and policy,
- reserve budgets,
- validate preconditions.

### 4.2 Execute
- perform effectful substeps under mediation,
- capture result hashes and outcome metadata.

### 4.3 Commit
- write durable state deltas,
- append receipt with transaction outcome,
- release unused budget reservation.

### 4.4 Rollback
- undo staged local deltas,
- append rollback receipt,
- mark external effects for compensation if needed.

## 5. Compensation model

Compensation applies to non-reversible operations:
- define compensation action (`undo`, `counter-action`, `operator-task`),
- attach compensation status in receipt chain,
- require explicit policy for auto-compensation vs manual escalation.

Compensation is mandatory for classes:
- outbound communication (`email.send`, `webhook.post`),
- financial/external state mutation,
- third-party configuration changes.

## 6. Failure semantics

The kernel classifies failures as:
- `policy_denied`
- `precondition_failed`
- `timeout`
- `partial_external_effect`
- `infra_error`

Each failure class must map to:
- retry policy,
- compensation requirement,
- escalation requirement,
- receipt outcome code.

## 7. Retry policy

Retries are allowed only when:
- idempotency is guaranteed by caller and target,
- retry budget permits,
- policy permits re-attempt class.

Blind retries without idempotency evidence are forbidden.

## 8. Invariants

- No mutating call without transaction context.
- No mutating call without idempotency key.
- No external side effect without outcome classification.
- No transaction terminal state without receipt.

## 9. Upstream
- [Agent kernel ABI](/blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md)
- [Policy language](/blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md)

## 10. Downstream
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Execution metering](/blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md)

## 11. Adjacent
- [Non-bypassable mediation](/blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## Conformance tests
- Duplicate mutating requests with same idempotency tuple do not create
  duplicate external effects.
- Rollback leaves no partial local state mutation.
- Irreversible external effects emit compensation-required status when rollback
  is impossible.
- Every terminal transaction state has a verifiable receipt.
