---
id: GONI-SPEC-DE0970FADC6E
title: 1.1 Clarification interrupts
type: specification
status: draft
implementation_state: specified_only
proposition: Clarification is a bounded interrupt subtype for delegation engineering.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 1.1 Clarification interrupts
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 1.1 Clarification interrupts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1.1 Clarification interrupts

Clarification is a bounded interrupt subtype for delegation engineering.

Clarification interrupts are allowed only when:

- a missing answer would change corridor or threshold outcome,
- a missing answer would change tool choice or counterparty/action target,
- a side effect is irreversible or costly enough that assumptions are
  insufficient,
- policy explicitly requires a user answer for the current task class.

Clarification interrupts are not allowed when the missing information can be
derived from active policy, retrieved context, prior approvals, or stable task
defaults. In those cases the runtime must proceed under surfaced assumptions or
block/escalate the action.

The scheduler MUST track clarification budgets and cooldowns separately from
general solver wakes so an over-questioning agent cannot degrade the operator
experience.
