---
id: GONI-SPEC-FEDFE25D0BD2
title: 2.1 ITCR budgets
type: specification
status: draft
implementation_state: specified_only
proposition: 'When an interrupt escalates into an ITCR episode (ITCR-01), the scheduler attaches hard budgets: max_wall_time_ms max_candidate_expansions max_planning_depth max_tool_planning_depth max_tokens_total Budget exhaustion terminates the episode and is recorded in audit logs.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 2.1 ITCR budgets
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 2.1 ITCR budgets

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1 ITCR budgets

When an interrupt escalates into an ITCR episode (ITCR-01), the scheduler
attaches hard budgets:

- max_wall_time_ms
- max_candidate_expansions
- max_planning_depth
- max_tool_planning_depth
- max_tokens_total

Budget exhaustion terminates the episode and is recorded in audit logs.
