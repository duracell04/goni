---
id: GONI-SYNTHESIS-C163EFE0F1BB
title: 1) Expand the truth map into a system graph
type: synthesis
status: draft
implementation_state: specified_only
proposition: Add entries for key docs, schemas, kernel crates, APIs, tests, and harnesses.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 1) Expand the truth map into a system graph
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 1) Expand the truth map into a system graph

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1) Expand the truth map into a system graph
- Add entries for key docs, schemas, kernel crates, APIs, tests, and harnesses.
- Add typed edges: `implements`, `tested_by`, `depends_on`.
- Extend `blueprint/scripts/validate_truth_map.py` to enforce:
  - required edges for `role: spec` (unless explicitly exempted)
  - no orphan entries for must-connect roles
