---
id: GONI-SPEC-E51871F72C7C
title: 4. Budget capabilities
type: specification
status: draft
implementation_state: specified_only
proposition: 'Budgets are first-class policy constraints: max_calls max_bytes_out max_cpu_ms max_wall_ms max_cost_units Budget exhaustion is a deny condition with structured decision output.'
domains:
- kernel
- policy
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md
  heading: 4. Budget capabilities
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 4. Budget capabilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Budget capabilities

Budgets are first-class policy constraints:
- `max_calls`
- `max_bytes_out`
- `max_cpu_ms`
- `max_wall_ms`
- `max_cost_units`

Budget exhaustion is a deny condition with structured decision output.
