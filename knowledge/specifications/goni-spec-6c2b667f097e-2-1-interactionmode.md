---
id: GONI-SPEC-6C2B667F097E
title: 2.1 InteractionMode
type: specification
status: draft
implementation_state: specified_only
proposition: 'InteractionMode = delegation | co_creation delegation: the user objective is recoverable from context, policy, or stable defaults.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.1 InteractionMode
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.1 InteractionMode

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 InteractionMode

`InteractionMode = delegation | co_creation`

- `delegation`: the user objective is recoverable from context, policy, or
  stable defaults.
- `co_creation`: the objective itself is materially ambiguous and must be
  narrowed before execution.
