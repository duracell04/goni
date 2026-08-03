---
id: GONI-SYNTHESIS-0DE23F800F2A
title: Validation checklist
type: synthesis
status: draft
implementation_state: specified_only
proposition: python blueprint/scripts/validate_truth_map.py python blueprint/scripts/generate_agents.py bash blueprint/scripts/txt_lint.sh python blueprint/goni-lab/goni_lab.py bench --scenario blueprint/goni-lab/scenarios/mixed.json
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: Validation checklist
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# Validation checklist

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Validation checklist
- `python blueprint/scripts/validate_truth_map.py`
- `python blueprint/scripts/generate_agents.py`
- `bash blueprint/scripts/txt_lint.sh`
- `python blueprint/goni-lab/goni_lab.py bench --scenario blueprint/goni-lab/scenarios/mixed.json`
