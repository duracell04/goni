---
id: GONI-SPEC-A78709639E40
title: 5. Budget enforcement
type: specification
status: draft
implementation_state: specified_only
proposition: Runtime MUST enforce hard and soft budgets at policy gate boundaries.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: 5. Budget enforcement
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 5. Budget enforcement

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Budget enforcement
- Runtime MUST enforce hard and soft budgets at policy gate boundaries.
- Exceeding hard budgets MUST stop further remote/model execution.
