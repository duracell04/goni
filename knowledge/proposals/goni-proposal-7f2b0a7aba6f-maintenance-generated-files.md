---
id: GONI-PROPOSAL-7F2B0A7ABA6F
title: Maintenance (Generated Files)
type: proposal
status: draft
implementation_state: specified_only
proposition: Do not edit AGENTS.md files directly (they are generated).
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/AGENTS.md
  heading: Maintenance (Generated Files)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Maintenance (Generated Files)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Maintenance (Generated Files)
- Do not edit `AGENTS.md` files directly (they are generated).
- Edit templates under `blueprint/docs/meta/` (for example `blueprint/docs/meta/agents.root.template.md`).
- Update canonical paths in `blueprint/docs/meta/truth-map.json` if files move.
- Regenerate AGENTS with `goni-prototype-lab:scripts/generate_agents.py`.
- Validate the truth map with `goni-prototype-lab:scripts/validate_truth_map.py`.
