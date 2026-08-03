---
id: GONI-SYNTHESIS-B738AB6F3F52
title: 2) Action Card
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Purpose: explicit propose-before-act unit for side-effectful work.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/10-primitives.md
  heading: 2) Action Card
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2) Action Card

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2) Action Card
- Purpose: explicit propose-before-act unit for side-effectful work.
- Contract anchor: `schemas/cards/action_card.schema.json`.
- Core invariant: action requires visible intent, required capabilities, and status transitions.
- Metrics: TTFA, approve/edit/reject rates, undo rate, execution success.
