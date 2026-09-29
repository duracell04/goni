---
id: SCHED-01
title: SCHED-01 - Scheduler and Interrupts
type: specification
status: draft
implementation_state: specified_only
proposition: "\uFEFF--- id: SCHED-01 type: SPEC status: specified_only DOC-ID: SCHED-01 Status: Specified only / roadmap This spec defines when and how the kernel escalates work from continuous, low-power cognition to expensive solver/LLM interrupts."
domains:
- specs
aliases:
- SCHEDULER-AND-INTERRUPTS
relations:
- type: depends_on
  target: TIME-01
  note: Deadline-driven scheduling and escalation consume temporal-authority semantics rather than treating every date as an equivalent movable timestamp.
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: SCHED-01 - Scheduler and Interrupts
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# SCHED-01 - Scheduler and Interrupts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# SCHED-01 - Scheduler and Interrupts
﻿---
id: SCHED-01
type: SPEC
status: specified_only
---
DOC-ID: SCHED-01
Status: Specified only / roadmap

This spec defines when and how the kernel escalates work from continuous,
low-power cognition to expensive solver/LLM interrupts.

## Temporal-authority scheduling

Deadline risk is authority-sensitive. The scheduler consumes TIME-01 temporal
constraints when a real-world date materially affects admission, priority,
replanning, or escalation.

The scheduler MUST distinguish at least these cases:

- **TARGET slip:** an internally controlled target may be replanned within the
  applicable autonomy corridor. Replanning MUST preserve any linked operative
  deadline and recompute the remaining safety margin.
- **COMMITTED breach risk:** the scheduler may prioritize work, surface risk,
  or route a proposal into the relevant negotiation/approval path. Ordinary
  scheduling authority MUST NOT mutate the committed deadline itself.
- **FIXED breach risk:** the scheduler treats the date as a protected external
  boundary. It may reprioritize movable work around the boundary or trigger an
  authorized extension/exception workflow, but MUST NOT resolve overload by
  moving the fixed deadline.
- **UNKNOWN authority:** if classification would materially alter safe
  execution, the scheduler preserves the uncertainty and routes retrieval,
  conservative handling, or a clarification interrupt rather than assuming the
  date is movable.

A scheduler-facing `JobSpec.deadline` remains distinct from these real-world
constraints. It may be derived from them, but it does not replace them as the
source of authority semantics.

### Deadline-risk audit basis

When deadline risk materially changes scheduling or escalation, the scheduling
decision SHOULD preserve the relevant `temporal_constraint_refs` through the
Work Order and audit/receipt path so later review can reconstruct why the
scheduler treated one date as movable and another as protected.
