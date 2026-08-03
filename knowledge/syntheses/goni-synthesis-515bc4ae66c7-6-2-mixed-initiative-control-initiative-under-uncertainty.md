---
id: GONI-SYNTHESIS-515BC4AE66C7
title: 6.2 Mixed-initiative control (initiative under uncertainty)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Mixed-initiative systems allocate initiative between user and system under uncertainty.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.2 Mixed-initiative control (initiative under uncertainty)
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.2 Mixed-initiative control (initiative under uncertainty)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.2 Mixed-initiative control (initiative under uncertainty)

Mixed-initiative systems allocate initiative between user and system under
uncertainty. The system's initiative must be scoped to confidence and preserve
user sovereignty via deferral and override mechanisms. [R1]

Goni mapping (normative):
- Action Cards: propose -> approve/deny/defer -> execute.
- Daily Brief: default deferral/batching surface.
- Agent manifests: declared scope, triggers, and budgets constrain initiative.
- Capability-checked syscalls: initiative cannot bypass governance.
