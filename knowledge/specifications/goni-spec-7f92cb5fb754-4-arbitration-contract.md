---
id: GONI-SPEC-7F92CB5FB754
title: 4. Arbitration contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'The kernel enforces the following sequence for any state mutation or tool call: Proposal Inputs: current state_snapshot_id, provenance, and a candidate list of: state deltas (F_sparse patches, optional S_core updates) tool calls (with args) Proposals may originate from encoders, agents, or tools.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 4. Arbitration contract
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 4. Arbitration contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Arbitration contract

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
