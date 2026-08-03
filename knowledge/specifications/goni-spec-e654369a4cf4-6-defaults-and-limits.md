---
id: GONI-SPEC-E654369A4CF4
title: 6. Defaults and limits
type: specification
status: draft
implementation_state: specified_only
proposition: 'Unless stricter policy overrides them, the runtime defaults are: at most one decisive question per task, at most two objective options when `clarification_decision = propose_objectives`, one target output before optional transformations, explicit surfaced assumptions whenever execution proceeds without asking, audit-grade mode is sticky inside the same task/session until reset.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 6. Defaults and limits
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 6. Defaults and limits

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Defaults and limits

Unless stricter policy overrides them, the runtime defaults are:

- at most one decisive question per task,
- at most two objective options when `clarification_decision =
  propose_objectives`,
- one target output before optional transformations,
- explicit surfaced assumptions whenever execution proceeds without asking,
- audit-grade mode is sticky inside the same task/session until reset.
