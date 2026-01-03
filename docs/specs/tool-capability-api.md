# TOOL-01 - Tool Capability API (Auditable Syscalls)
DOC-ID: TOOL-01
Status: Draft (normative target)

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

See `software/50-data/51-schemas-mvp.md` for `AuditRecords` and `CapabilityTokens`.

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
- `vecdb.query(embedding, filters, cap)`
- `calendar.find(range, cap)`
- `email.draft(to, subject, body, cap)`

## 6. Invariants

- **No bypass:** tools cannot be called without a capability token.
- **Auditability:** every tool call produces an audit record.
- **Policy mediation:** policy engine is the sole authority for tool approval.

## 7. Related specs

- `docs/specs/agent-definition.md`
- `docs/specs/latent-state-contract.md`
- `docs/specs/scheduler-and-interrupts.md`
- `docs/specs/symbolic-substrate.md`
