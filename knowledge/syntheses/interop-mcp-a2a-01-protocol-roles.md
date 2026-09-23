---
id: INTEROP-MCP-A2A-01
title: MCP and A2A occupy different interoperability roles
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni treats MCP primarily as a vertical capability and context interoperability boundary and A2A as a horizontal agent-system interoperability boundary; neither protocol grants Goni authority by itself.
domains:
- system
- interoperability
- tools
- agents
aliases:
- MCP versus A2A
relations:
- type: refines
  target: GONI-SYNTHESIS-8BBCE3886093
sources:
- SRC-MCP-2026-07-28
- SRC-A2A-1-0
artifacts: []
uncertainty: Protocol capabilities evolve; implementation should pin supported versions and avoid treating the vertical/horizontal shorthand as a statement that either protocol can only be used in one topology.
legacy: []
---

# MCP and A2A occupy different interoperability roles

Goni uses a simple mental model:

```text
                    A2A
Goni agent/runtime <----> independent agent/runtime
       |
       | MCP
       v
tools / resources / contextual capabilities
```

**MCP** primarily standardizes how an AI application discovers and interacts with capabilities and contextual resources exposed by servers. The protocol is richer than ordinary function calling because its model includes multiple server primitives and negotiated capabilities.

**A2A** primarily standardizes task-oriented interaction among independently implemented agent systems. It allows an agent to expose capabilities and exchange task progress or artifacts without granting peers access to its internal prompt, memory, or tool graph.

For Goni, both are adapters outside the authority root. A protocol peer can advertise a capability or return an artifact; the kernel still decides what may be sent, what may be trusted, what actions are authorized, and what receipts or verification are required.
