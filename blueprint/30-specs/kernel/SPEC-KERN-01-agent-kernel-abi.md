---
id: SPEC-KERN-01
type: SPEC
status: specified_only
---
# SPEC-KERN-01 - Agent Kernel ABI
DOC-ID: SPEC-KERN-01
Status: Specified only / roadmap

This spec defines the mandatory kernel boundary for all effectful agent
operations. It is the reference-monitor choke point for tool actions, network
egress, and state mutation proposals.

## 1. Design goal

The ABI makes complete mediation testable:
- no effectful operation is valid unless submitted through this interface,
- all approved/denied operations emit receipts,
- capabilities and policy decisions are explicit ABI inputs.

## 2. Core objects

### 2.1 Job descriptor

Required fields:
- `job_id`
- `job_class` (`interactive` | `background` | `maintenance`)
- `slo_profile`
- `budget_set`
- `cancel_policy`
- `capability_set_id`
- `trace_id`

### 2.2 Capability handle

Capability handles are opaque references to policy-issued authority:
- unforgeable by userland tool code,
- scoped to resource/action sets,
- time-bounded and revocable.

### 2.3 Transaction descriptor

Required fields:
- `tx_id`
- `operation_id`
- `idempotency_key` (required for mutating calls)
- `preconditions`

## 3. Agent syscalls (logical ABI)

### 3.1 `tool_call`

`tool_call(cap_handle, action, args_ref, tx_id, idempotency_key) -> outcome`

Outcome envelope includes:
- `status` (`ok` | `denied` | `error` | `cancelled`)
- `result_ref` (or error ref)
- `receipt_ref` (mandatory)
- `policy_decision_ref`

### 3.2 `propose_state_delta`

`propose_state_delta(tx_id, delta_ref, provenance_ref) -> proposal_ref`

No user-visible state change is committed by this call alone.

### 3.3 `commit_tx`

`commit_tx(tx_id) -> {status, receipt_ref}`

Applies staged effects if preconditions and policy still hold.

### 3.4 `rollback_tx`

`rollback_tx(tx_id, reason) -> {status, receipt_ref}`

Rolls back staged operations and records failure/deny semantics.

## 4. Effect classes

Effects requiring kernel mediation:
- external network calls,
- filesystem writes and deletes,
- connector side effects (email send, calendar mutation, payments),
- mutable state commits in local durable stores,
- externally visible job/control-plane actions.

Read-only in-memory transformations without external side effects may bypass
transaction commit, but not capability checks when protected resources are read.

## 5. Mandatory receipt semantics

Every `tool_call`, `commit_tx`, and `rollback_tx` MUST emit exactly one receipt.

Receipt minimum:
- `trace_id`
- `span_id`
- `job_id`
- `tx_id`
- `capability_id`
- `policy_decision`
- `action_type`
- `timestamp`

See `blueprint/30-specs/receipts.md` for canonical receipt fields.

## 6. Cancellation semantics

The ABI supports three cancellation points:
- `immediate`: cancel before any effectful substep starts,
- `after_token`: cancel at model token boundary,
- `after_tool_call`: cancel once current mediated tool action completes.

Cancellation policy is declared on job submission and logged in receipts.

## 7. Invariants

- No ambient authority: tools cannot perform privileged action without a valid
  capability handle.
- No silent effect: every effectful action yields a receipt reference.
- No invisible policy: allow/deny basis is machine-readable.
- No duplicate mutation: idempotency keys prevent repeated side effects.

## 8. Upstream
- [Policy language](/blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md)
- [Transactional tools](/blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md)
- [Job contract](/blueprint/30-specs/job.md)

## 9. Downstream
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)

## 10. Adjacent
- [Non-bypassable mediation](/blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)

## Conformance tests
- No outbound network bytes occur unless issued through `tool_call` with an
  authorized net capability.
- Every successful, denied, failed, and cancelled `tool_call` emits one receipt.
- Mutating calls without `idempotency_key` are rejected.
- Cancellation behavior matches declared cancellation policy.
