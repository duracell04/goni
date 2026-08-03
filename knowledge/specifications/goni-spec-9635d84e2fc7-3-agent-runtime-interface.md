---
id: GONI-SPEC-9635D84E2FC7
title: 3. Agent runtime interface
type: specification
status: draft
implementation_state: specified_only
proposition: 'The agent runtime exposes a restricted API surface: read_state(snapshot_id) -> StateSnapshot retrieve(evidence_request) -> EvidenceBatch request_solver(request) -> SolverHandle call_tool(tool_id, args, capability_token) -> ToolResult propose_commit(state_delta, artifacts) -> CommitId The runtime itself is untrusted; the kernel validates all effects.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-definition.md
  heading: 3. Agent runtime interface
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 3. Agent runtime interface

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Agent runtime interface

The agent runtime exposes a restricted API surface:

- `read_state(snapshot_id) -> StateSnapshot`
- `retrieve(evidence_request) -> EvidenceBatch`
- `request_solver(request) -> SolverHandle`
- `call_tool(tool_id, args, capability_token) -> ToolResult`
- `propose_commit(state_delta, artifacts) -> CommitId`

The runtime itself is untrusted; the kernel validates all effects.
