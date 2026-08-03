---
id: GONI-SPEC-4751F23CD0FD
title: 4. ITCR budgets (hard limits)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every ITCR episode executes within explicit budgets: max_wall_time_ms max_candidate_expansions max_planning_depth max_tool_planning_depth max_tokens_total Budget violations terminate the episode with a failed or partial verdict.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 4. ITCR budgets (hard limits)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 4. ITCR budgets (hard limits)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. ITCR budgets (hard limits)

Every ITCR episode executes within explicit budgets:

- max_wall_time_ms
- max_candidate_expansions
- max_planning_depth
- max_tool_planning_depth
- max_tokens_total

Budget violations terminate the episode with a failed or partial verdict.
