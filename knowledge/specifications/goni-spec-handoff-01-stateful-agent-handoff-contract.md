---
id: GONI-SPEC-HANDOFF-01
title: "Stateful Agent Handoff Contract"
type: specification
status: draft
implementation_state: specified_only
proposition: "Cross-agent and cross-context handoffs should transmit canonical task state, authority context, evidence references, completed work, open issues, and next permitted actions rather than relying on replay of the complete conversational or cognitive history."
domains:
- agent
- memory
- specs
aliases: []
relations:
- type: refines
  target: GONI-SPEC-C0EC9D0B0A06
- type: depends_on
  target: DELEG-INT-01
- type: depends_on
  target: GONI-SPEC-5211A9E877AD
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: "The exact serialized schema is intentionally left open until an implementation lane validates the minimum sufficient handoff payload."
legacy: []
---

# Stateful Agent Handoff Contract

A handoff is a state transfer, not a transcript transfer.

A minimum handoff object SHOULD reference the Work Order and Done Contract, current StateSnapshot, completed action and receipt references, evidence and artifact references, unresolved questions and blockers, active authority or corridor context, and next permitted actions.

Raw conversation or trajectory history MAY be attached when it contains task-relevant information that canonical state has not preserved. It is not the default state carrier.
