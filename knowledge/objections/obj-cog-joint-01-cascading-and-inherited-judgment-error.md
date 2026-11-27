---
id: OBJ-COG-JOINT-01
title: 'Objection: confident joints can inherit incorrect state'
type: objection
status: draft
implementation_state: not_applicable
proposition: Decomposing cognition into fast typed decisions can increase system-level error through stale
  state, lost dependencies, distribution shift or correlated mistakes even when individual outputs are confidently
  typed.
domains:
- agent
- research
- evaluation
- kernel
aliases: []
relations:
- type: objects_to
  target: COG-JOINT-01
  note: Module accuracy and schema validity alone do not justify deploying a distributed graph.
- type: objects_to
  target: COG-SPEC-01
  note: Lower inference cost can be offset by compounded errors and rework.
sources:
- SRC-YANG2026-JEV-LIMITS
- SRC-IMPERATRICE2026-JEV-BIAS
- SRC-LI2026-JEV-COMPARISON
- SRC-GEIFMAN2017-SELECTIVE
artifacts: []
uncertainty: These are external reported findings and plausible system risks. Failure magnitude and mitigation
  effectiveness in Goni remain unknown.
legacy: []
---

# Objection: confident joints can inherit incorrect state

Yang et al. (2026 preprint) report that evaluated Jev-style single-pass
models handled large record lookup tasks but deteriorated when resolving
multiple updates to the same entity required state reconstruction.
Given stale summaries, their choices followed the supplied premise;
evaluated confidence and handoff approaches did not consistently detect
inherited state errors. Imperatrice et al. report sensitivity to option order,
irrelevant numeric inputs, and reformulations, including logical consistency
failures across equivalent conjunction tasks. Li et al. report a comparative
weakness on multi-step mathematical tasks.

The implication is sharper than ordinary confidence calibration: a high
score conditioned on a wrong state can be misleading. Separately accurate
joints can also generate correlated mistakes or fail because a globally
necessary dependency was lost during decomposition. Mechanical JSON/enum
validity does not express task correctness. A larger deliberative model can
also be wrong.

Evaluation must therefore compare whole-task success and error loss against
the monolithic and modular baselines. Include stale/conflicting evidence,
unknown options, option permutations, semantically equivalent formulations,
long dependent sequences, missing context, changed work orders, and
distribution shift. Reconstruct state from authoritative current records
where required; treat summaries as cognitive aids. Use independently
validated risk/coverage thresholds, abstention, bounded deliberation and
human review for unresolved judgments. Never derive effect authorization
from a confidence number.

This objection remains open until the relevant experiment produces pinned
evidence. Prefer a simpler implementation when coordination overhead or
failure propagation defeats specialization.
