---
id: GONI-EXPERIMENT-17AD373C1ACF
title: Required replay scenarios
type: experiment
status: draft
implementation_state: not_applicable
proposition: A single correction creates a scoped hypothesis, not a global preference.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-HARNESS-02-correction-delta-compiler.md
  heading: Required replay scenarios
  revision: f91b4339e701c60c0d3508cf2bb3bd613ef2e36d
---

# Required replay scenarios

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Required replay scenarios

- A single correction creates a scoped hypothesis, not a global preference.
- Repeated consistent corrections increase confidence only within matching
  scope.
- Contradictory corrections reduce confidence, narrow scope, or require review.
- Accepted learning emits `MemoryEntry + Receipt + RegressionTest`.
- Learning receipts omit raw user, draft, and correction text by default.
- High-risk, privacy, legal, financial, or constitutional preferences require
  explicit approval before promotion.
- Retrieval, prompt, routing, or policy behavior changes attach only to declared
  Learning Loop seams.
- Replayed tasks show lower correction distance without higher rejection,
  override, interruption, policy violation, or privacy-risk rates.
