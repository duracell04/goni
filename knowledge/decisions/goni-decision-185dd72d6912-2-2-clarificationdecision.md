---
id: GONI-DECISION-185DD72D6912
title: 2.2 ClarificationDecision
type: decision
status: draft
implementation_state: specified_only
proposition: 'ClarificationDecision = assume | ask_decisive | propose_objectives | block assume: proceed under surfaced assumptions.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.2 ClarificationDecision
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.2 ClarificationDecision

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 ClarificationDecision

`ClarificationDecision = assume | ask_decisive | propose_objectives | block`

- `assume`: proceed under surfaced assumptions.
- `ask_decisive`: ask one bounded question whose answer materially changes the
  outcome.
- `propose_objectives`: surface at most two candidate objectives and require
  user selection.
- `block`: refuse execution because safe progress is not possible.
