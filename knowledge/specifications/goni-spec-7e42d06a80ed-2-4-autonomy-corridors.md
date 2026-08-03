---
id: GONI-SPEC-7E42D06A80ED
title: 2.4 Autonomy corridors
type: specification
status: draft
implementation_state: specified_only
proposition: 'Each task_class MUST declare one of: no_go: never execute automatically.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 2.4 Autonomy corridors
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 2.4 Autonomy corridors

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.4 Autonomy corridors

Each `task_class` MUST declare one of:

- `no_go`: never execute automatically.
- `soft_gate`: execute only with bounded conditions and review gates.
- `autopilot`: execute by default without pre-approval.

These are the three kernel authority corridors. Corridor assignment is
policy data, versioned, and owned by the Goni kernel rather than by external
assistant frameworks or gateways.

For operator-facing discussion, Goni may describe these as:

- `autopilot`: execute inside the active risk threshold,
- `soft_gate`: require lightweight approval or bounded review,
- `hard_gate`: require explicit human decision.

`no_go` remains the deny-only policy floor in the normative corridor schema.
