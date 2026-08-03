---
id: GONI-SPEC-EAF9510BD81E
title: 1.2 Co-creation interrupts
type: specification
status: draft
implementation_state: specified_only
proposition: Co-creation is a separate interrupt subtype for unresolved objective choice.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 1.2 Co-creation interrupts
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 1.2 Co-creation interrupts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2 Co-creation interrupts

Co-creation is a separate interrupt subtype for unresolved objective choice.

Co-creation interrupts are allowed only when:

- two or more materially different objectives remain plausible,
- selecting among them would define the user's goal rather than execute it,
- policy does not permit silent defaulting for the current task class.

Co-creation interrupts are not allowed for merely missing factual details. In
those cases the runtime must remain in delegation mode and choose among
`assume`, `ask_decisive`, or `block`.

The scheduler MUST track co-creation interrupts separately from clarification
interrupts so goal ambiguity is not conflated with missing-parameter lookup.
