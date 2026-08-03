---
id: GONI-DECISION-FE4388561A24
title: Context
type: decision
status: draft
implementation_state: specified_only
proposition: Local inference-time compute reasoning (ITCR) is constrained by memory bandwidth, accelerator graph constraints, storage endurance, and thermal dynamics.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Context
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Context

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Context

Local inference-time compute reasoning (ITCR) is constrained by memory
bandwidth, accelerator graph constraints, storage endurance, and thermal
dynamics. These constraints must be encoded as hardware platform contracts so
software scheduling and routing can enforce them.
