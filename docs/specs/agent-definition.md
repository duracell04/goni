# AGENT-01 - Agent Definition (Goni OS)
DOC-ID: AGENT-DEF-01
Status: Specified only / roadmap

This document defines what an agent is in Goni OS and how it interacts with the
kernel. Agents are userland processes; cognition is a kernel substrate; solver
calls are budgeted interrupts.

## 1. Definition

A Goni OS Agent is a **local userland process** that:

- reads kernel-maintained latent state,
- invokes capability-scoped system calls (tools),
- and requests solver/LLM compute only when escalation is justified.

Agents do not own global state and have no ambient authority.

## 2. Required invariants

- **Policy mediation:** every tool call and state write is policy-checked and
  audited.
- **Latent-first loop:** continuous cognition does not require LLM decoding.
- **Budget enforcement:** solver calls, CPU/GPU time, and disk writes are
  quota-governed.
- **Crash consistency:** state can be reconstructed from snapshots + deltas.

## 3. Agent runtime interface

The agent runtime exposes a restricted API surface:

- `read_state(snapshot_id) -> StateSnapshot`
- `retrieve(evidence_request) -> EvidenceBatch`
- `request_solver(request) -> SolverHandle`
- `call_tool(tool_id, args, capability_token) -> ToolResult`
- `propose_commit(state_delta, artifacts) -> CommitId`

The runtime itself is untrusted; the kernel validates all effects.

## 4. Audit fields

Every agent action that causes side effects must be attributable by:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

These fields are required on tool calls, state deltas, and commits.

## 5. Related specs

- [latent-state-contract.md](./latent-state-contract.md)
- [agent-manifest.md](./agent-manifest.md)
- [tool-capability-api.md](./tool-capability-api.md)
- [scheduler-and-interrupts.md](./scheduler-and-interrupts.md)
- [itcr.md](./itcr.md)

## Conformance tests
- TBD: add tests for this spec.



