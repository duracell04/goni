---
id: TOOL-01
type: SPEC
status: specified_only
---
# TOOL-01 - Tool Capability API (Auditable Syscalls)
DOC-ID: TOOL-01
Status: Specified only / roadmap

Tools are kernel-mediated, capability-scoped syscalls. All tool invocations are
audited and attributable to an agent and a state snapshot.

## 1. Tool call envelope

Logical fields for every tool call:

- `tool_id`
- `args`
- `agent_id`
- `capability_token_id`
- `state_snapshot_id`
- `policy_hash`
- `provenance`
- `operation_id`
- `task_class`
- `autonomy_mode` (`no_go` | `soft_gate` | `autopilot` | `escalated`)
- `risk_score`
- `risk_basis`
- `idempotency_key` (required for mutating calls)
- `precondition_refs` (state/version references that must hold at commit time)

The kernel validates the call against policy and capability tokens before
execution.
Validation follows the SS-01 arbitration contract (symbolic constraints in F_sparse).

## 2. Tool result envelope

Tool results MUST include:

- `tool_id`
- `agent_id`
- `state_snapshot_id`
- `status` (ok | error)
- `result_hash`
- `provenance`
- `operation_id`
- `task_class`
- `autonomy_mode`
- `risk_score`
- `tx_outcome` (`committed` | `rolled_back` | `no_op`)
- `commit_delta_id` (present when `tx_outcome = committed`)

## 2.1 Transactional tool semantics (normative)

All mutating tool calls MUST execute as mediated transactions:

- **Prepare/precondition check:** validate capability token and policy hash,
  validate `precondition_refs` against current state/version, and reserve
  budget according to policy.
- **Commit:** apply side effects atomically, append state delta(s), and emit
  audit record + receipt with resulting hashes.
- **Rollback:** on policy rejection, failed preconditions, or commit failure,
  no partial side effects may remain; emit an auditable failure result
  (`tx_outcome = rolled_back`).

Replay and idempotency rules:
- mutating calls MUST include `idempotency_key`,
- repeated `(tool_id, idempotency_key, args_hash, capability_token_id)` MUST
  return the original outcome without duplicate side effects,
- stale/invalid replay attempts MUST fail closed and remain auditable.

## 3. Canonical audit record

Audit records are written for every tool call and state mutation. Required
fields:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `capability_token_id`
- `tool_id`
- `args_hash`
- `result_hash`
- `provenance`
- `task_class`
- `autonomy_mode`
- `risk_score`
- `risk_basis`

See `blueprint/software/50-data/51-schemas-mvp.md` for `AuditRecords` and `CapabilityTokens`.

## 4. Capability tokens

Capability tokens bind:

- allowed tool IDs,
- resource scopes (filesystem roots, network domains),
- budgets (tokens, CPU time, disk writes),
- expiry and revocation rules.

Tokens are immutable and referenced by ID in tool calls.

## 5. Example syscalls (non-exhaustive)

- `fs.read(path, cap)`
- `fs.write(path, bytes, cap)`
- `net.egress(route, purpose, payload_ref, cap)`
- `vecdb.query(embedding, filters, cap)`
- `calendar.find(range, cap)`
- `email.draft(to, subject, body, cap)`

## 6. Invariants

- **No bypass:** tools cannot be called without a capability token.
- **Auditability:** every tool call produces an audit record.
- **Policy mediation:** policy engine is the sole authority for tool approval.
- **Delegation mediation:** autonomy corridor and risk thresholds are evaluated
  before execution.
- **Transactional safety:** mutating calls are atomic (commit or rollback).
- **Replay safety:** idempotency keys prevent duplicate side effects.

## 7. Related specs

- [agent-definition.md](/blueprint/30-specs/agent-definition.md)
- [latent-state-contract.md](/blueprint/30-specs/latent-state-contract.md)
- [scheduler-and-interrupts.md](/blueprint/30-specs/scheduler-and-interrupts.md)
- [symbolic-substrate.md](/blueprint/30-specs/symbolic-substrate.md)
- [delegation-and-autonomy.md](/blueprint/30-specs/delegation-and-autonomy.md)

## 8. Upstream
- [Symbolic substrate](/blueprint/30-specs/symbolic-substrate.md)
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Agent definition](/blueprint/30-specs/agent-definition.md)

## 9. Downstream
- [Receipts](/blueprint/30-specs/receipts.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Orchestrator](/blueprint/software/30-components/orchestrator.md)

## 10. Adjacent
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)
- [System map](/blueprint/docs/00-system-map.md)

## Conformance tests
- TBD: add tests for this spec.






