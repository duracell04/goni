---
id: EVID-A2A-INTEROPERABILITY-01
title: 'Source claim: A2A connects independent agent systems'
type: evidence
status: draft
implementation_state: not_applicable
proposition: The Agent2Agent protocol defines interoperable task-oriented communication between independent agent systems, including discovery or capability description and exchange of messages, tasks, status, and artifacts.
domains:
- research
- interoperability
- agents
aliases: []
relations:
- type: supports
  target: INTEROP-MCP-A2A-01
sources:
- SRC-A2A-1-0
artifacts: []
uncertainty: A2A is a protocol boundary, not evidence that multi-agent decomposition is beneficial for a particular Goni workload.
legacy: []
---

# Source claim: A2A connects independent agent systems

A2A is designed for communication between agent systems that may be implemented independently and need not expose one another's internal prompts, memory, or tool wiring.

For Goni, A2A belongs on the horizontal agent-to-agent interoperability boundary, while Goni authority remains local and kernel-owned.
