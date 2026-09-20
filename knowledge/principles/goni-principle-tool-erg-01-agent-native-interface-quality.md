---
id: GONI-PRINCIPLE-TOOL-ERG-01
title: "Agent-Native Interface Quality"
type: principle
status: draft
implementation_state: specified_only
proposition: "Action-interface quality is part of system capability; Goni tools should reduce model inference burden through narrow typed operations, explicit state, deterministic identifiers, structured errors, idempotency, and auditable side effects."
domains:
- agent
- tools
aliases: []
relations:
- type: refines
  target: TOOL-01
- type: depends_on
  target: SPEC-TXN-01
sources:
- SRC-YANG2024-SWE-AGENT
artifacts: []
uncertainty: "Interface preferences are task-dependent; visual interaction remains necessary when the target system exposes no reliable structured action surface."
legacy: []
---

# Agent-Native Interface Quality

Tool ergonomics changes the reasoning burden placed on a model. Goni should prefer machine-legible action surfaces that make valid actions and failure states explicit.

For semantically equivalent integrations, the default preference is:

```text
typed API / IPC
> CLI
> structured UI / accessibility surface
> visual computer use
```

The preference reflects reliability and information density. Visual interaction remains an important compatibility layer.

High-quality action interfaces should expose deterministic identifiers, typed inputs and outputs, machine-readable errors, idempotency or explicit transactional semantics, and auditable side effects.
