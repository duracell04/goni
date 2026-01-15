# SS-01 - Symbolic Substrate (Arbitration Contract)
DOC-ID: SS-01
Status: Draft (normative target)
Conformance: TBD (goni-lab harness)

This spec defines the minimal symbolic substrate and arbitration contract that
keeps Goni OS auditable, enforceable, and deterministic. It is ABI-like: it
pins interface semantics without prescribing a specific rule engine or planner.

## 1. Purpose and scope

SS-01 defines:
- the terminology used in the symbolic layer,
- the arbitration contract for proposal -> validation -> execution -> commit,
- the authority model for who can write which symbolic namespaces, and
- minimal failure semantics and constraint payload shape.

Non-goals:
- full logic programming, HTN planners, or a complete policy language.
- storage layout changes (F_sparse remains map<utf8, utf8>).

## 2. Terminology

- fact: a keyed proposition stored in F_sparse (observed or derived).
- goal: a desired state or outcome stored in F_sparse.
- rule: a deterministic transform over symbolic state (facts/goals).
- constraint: a boolean guard that permits or blocks actions or state commits.
- invariant: a constraint that must hold after every commit.
- policy: a kernel-owned set of constraints and authorities.
- proposal: a candidate action or state delta from encoders, agents, or tools.
- validation: symbolic evaluation of a proposal against policy, facts, and
  schemas.

## 3. Arbitration contract

The kernel enforces the following sequence for any state mutation or tool call:

1) Proposal
   - Inputs: current state_snapshot_id, provenance, and a candidate list of:
     - state deltas (F_sparse patches, optional S_core updates)
     - tool calls (with args)
   - Proposals may originate from encoders, agents, or tools.

2) Validation (symbolic)
   - Policy engine evaluates:
     - capability tokens and scopes (TOOL-01)
     - constraints and invariants (this spec)
     - tool call schemas / types
     - conflict detection (e.g., mutually exclusive goals)
   - Output: allow | deny | defer, plus a reason code.

3) Execution (controlled)
   - Only allowed actions are scheduled (SCHED-01).
   - Tool calls are executed via the audited syscall envelope (TOOL-01).

4) Commit (append-only)
   - State deltas are appended to StateDeltas and checkpointed as needed.
   - Tool results and arbitration decisions are written to AuditRecords.

Invariants:
- No tool call or state mutation bypasses validation.
- All arbitration decisions are auditable and tied to state_snapshot_id.
- LLM output is advisory; only validated proposals can execute.

## 4. Authority model (F_sparse namespaces)

F_sparse is a map of namespaced keys. Authority is enforced by policy:

- policy.*     kernel-owned; only the policy engine can write.
- constraint.* kernel-owned; immutable except via policy updates.
- goal.*       agent-proposed; must pass validation before commit.
- fact.*       encoder/tool-derived; must pass validation before commit.

All writes occur through StateDeltas. Direct mutation is forbidden.

## 4.1 Confirmed vs speculation thresholds

For MemoryEntries:
- A claim is **confirmed** if `confirmed_by_event_id` is present, or if
  `source_chunk_ids` is non-empty, `confidence` meets the policy threshold, and
  `conflict_state` is not contradictory.
- Otherwise, the claim MUST be stored as `hypothesis` or `derived` with a
  `ttl_ms` or `review_at` value, and MUST NOT be promoted to `fact` without
  new evidence.

## 5. Failure semantics

When validation fails, the kernel must choose one of:
- block: deny action and record the failure in AuditRecords.
- ask: raise a user confirmation interrupt and defer execution.
- defer: request more evidence or schedule later.
- degrade: reduce capability/budget or switch modes (SCHED-01).

Failures never execute tool calls and never commit state changes.

## 6. Minimal constraint payload (v1)

Constraint values in F_sparse are versioned JSON objects. Minimal shape:

{
  "v": 1,
  "effect": "allow" | "deny",
  "subject": {"tool_id": "..."} | {"capability": "..."} | null,
  "when": {"op": "all|any|not|eq|in|exists|missing|schema", "args": [...]},
  "on_fail": "block" | "ask" | "defer",
  "reason": "optional text"
}

Evaluation rules:
- when/op predicates read from F_sparse by key or key.field path.
- schema predicates validate tool args or artifacts against a known schema.
- absent or malformed constraints are treated as deny by default.

Example constraints (JSON values stored under constraint.* keys):

- constraint.no_send_email:
  {"v":1,"effect":"deny","subject":{"tool_id":"email.send"},"on_fail":"block"}
- constraint.requires_source:
  {"v":1,"effect":"deny","subject":{"tool_id":"fs.write"},
   "when":{"op":"missing","args":["fact.source_ref"]},"on_fail":"ask"}

## 7. Related specs

- docs/specs/latent-state-contract.md
- docs/specs/tool-capability-api.md
- docs/specs/scheduler-and-interrupts.md
- docs/specs/agent-manifest.md
