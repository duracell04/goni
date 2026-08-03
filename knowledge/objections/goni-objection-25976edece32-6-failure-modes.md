---
id: GONI-OBJECTION-25976EDECE32
title: 6. Failure modes
type: objection
status: draft
implementation_state: not_applicable
proposition: 'Delegation quality MUST be evaluated against failure modes, not only task success: lazy_agent: asks the user for structure it could have inferred locally.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 6. Failure modes
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 6. Failure modes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Failure modes

Delegation quality MUST be evaluated against failure modes, not only task
success:

- `lazy_agent`: asks the user for structure it could have inferred locally.
- `overcautious_agent`: escalates or blocks routine work that fits active
  policy.
- `shape_shifter`: changes plan or rationale without surfacing the update.
- `complacency_engine`: proceeds confidently despite unresolved ambiguity.
- `hidden_assumption_executor`: makes materially important assumptions without
  exposing them.
- `goal_chooser`: silently selects among materially different user objectives
  instead of entering co-creation mode.

Policies, evals, and replay traces should be able to distinguish these modes so
fixes can attach to the right control seam.
