---
id: WORLD-EVAL-01
title: Predictive deliberation evaluation
type: experiment
status: draft
implementation_state: specified_only
proposition: A matched evaluation should measure whether bounded world modelling and
  adaptive simulation improve decisions at fixed resource budgets while preserving
  epistemic and authority boundaries.
domains:
- cognition
- evaluation
aliases: []
relations:
- type: tests
  target: WORLD-01
- type: tests
  target: SIM-BUDGET-01
- type: tests
  target: WORLD-MODEL-RISK-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Task suite, effect-size thresholds, probability calibration scope, and
  cost-normalized baselines remain to be selected.
legacy: []
---

# Predictive deliberation evaluation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

Compare direct decisions, fixed-depth simulation, and adaptive simulation on the
same partially observed tasks, authorized actions, models, and total budgets.
Include misleading observations, exogenous changes, actor-model errors,
correlated generated trajectories, stale evidence, and deadline/resource exhaustion.

Measure outcome utility/loss, reversibility exposure, inference/evidence costs,
latency, calibration where probabilities are claimed, abstention, and marginal
value-estimator error. Track simulated-to-observed promotions and authority
expansion as boundary failures. Record scenario seeds or stochastic test design,
implementation/model revisions, estimator settings, held-out outcomes, and
evidence lineage. Demonstrate decision benefit on held-out tasks against a named
baseline; acceptable effect size and uncertainty intervals must be set before
running. Planned cases are separate from future outcome evidence.
