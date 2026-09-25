---
id: EVID-PROGENT-01
title: 'Source claim: tool privileges can be enforced deterministically outside the LLM'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Progent applies programmable least-privilege policies to agent tool calls and enforces those policies deterministically outside the model reasoning loop.
domains: [research, security, tools]
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-DF627E719B9D
sources: [SRC-SHI2025-PROGENT]
artifacts: []
uncertainty: Progent's guarantees apply to its own architecture and benchmarks; Goni must independently implement and verify its capability boundary.
legacy: []
---

# Tool privileges can be enforced outside the LLM

Progent demonstrates the architectural pattern of a probabilistic agent proposing tool calls while a separate deterministic mechanism decides whether the proposed call is permitted.
