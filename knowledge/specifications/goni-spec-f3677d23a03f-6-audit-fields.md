---
id: GONI-SPEC-F3677D23A03F
title: 6. Audit fields
type: specification
status: draft
implementation_state: specified_only
proposition: 'Interrupt decisions and solver wakes are recorded with: agent_id (if an agent requested the interrupt) policy_hash state_snapshot_id provenance task_class autonomy_mode interaction_mode risk_score clarification_status clarification_decision work_order_id delegation_outcome'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 6. Audit fields
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 6. Audit fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Audit fields

Interrupt decisions and solver wakes are recorded with:

- `agent_id` (if an agent requested the interrupt)
- `policy_hash`
- `state_snapshot_id`
- `provenance`
- `task_class`
- `autonomy_mode`
- `interaction_mode`
- `risk_score`
- `clarification_status`
- `clarification_decision`
- `work_order_id`
- `delegation_outcome`
