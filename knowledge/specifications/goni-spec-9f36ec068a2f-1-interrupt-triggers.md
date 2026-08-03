---
id: GONI-SPEC-9F36EC068A2F
title: 1. Interrupt triggers
type: specification
status: draft
implementation_state: specified_only
proposition: 'Interrupts are raised on: surprisal > threshold goal_conflict detected by predictor explicit user intent policy deadlines or compliance conditions missing information that materially changes safe delegated execution unresolved objective ambiguity that requires co_creation Interrupts are routed through the scheduler and never call the solver directly.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 1. Interrupt triggers
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 1. Interrupt triggers

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Interrupt triggers

Interrupts are raised on:

- `surprisal > threshold`
- `goal_conflict` detected by predictor
- explicit user intent
- policy deadlines or compliance conditions
- missing information that materially changes safe delegated execution
- unresolved objective ambiguity that requires `co_creation`

Interrupts are routed through the scheduler and never call the solver directly.
Admission is part of the SS-01 arbitration contract.
