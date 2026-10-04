---
id: SIM-BUDGET-01
title: Decision-aware simulation allocation
type: specification
status: draft
implementation_state: specified_only
proposition: Additional simulation is eligible when its estimated decision benefit
  exceeds its full marginal cost within the existing permission, resource, deadline,
  and stopping constraints.
domains:
- cognition
- scheduler
- evaluation
aliases: []
relations:
- type: refines
  target: DREAM-01
- type: refines
  target: SCHED-01
- type: depends_on
  target: WORLD-01
- type: depends_on
  target: EPI-CTRL-01
- type: depends_on
  target: LOSS-01
- type: depends_on
  target: METER-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Value-of-computation estimates, cost accounting, stopping thresholds,
  and scheduling utility require matched empirical evaluation.
legacy: []
---

# Decision-aware simulation allocation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Marginal value

This contract refines simulation-specific allocation within the existing
epistemic controller, DREAM-01, and scheduler; it reuses their budgets and
completion/stopping lifecycle.

For an eligible additional simulation \(c\), the conceptual net value is:

\[
VOC(c\mid b_t)=\mathbb E[\max_{a\in A_{allowed}} Q(a,b'_t)]
-\max_{a\in A_{allowed}}Q(a,b_t)-Cost_{total}(c).
\]

\(Q\) is the named decision-value profile supplied through WORLD-01/LOSS-01.
The baseline is the best decision with currently available evidence. The
marginal cost includes inference, evidence, verification, transfer, latency,
energy, and opportunity cost where relevant, with each contribution counted
once. \(b'_t\) is the updated cognitive representation after the simulation.

\[
Execute(c)\iff Eligible(c)\land\widehat{VOC}(c)>0.
\]

Eligibility includes current authority, remaining resources, finite branch and
horizon limits, deadline feasibility, and the existing DoneContract. The
scheduler retains final resource arbitration.

## Estimation and termination

A conforming profile MUST name its estimator, uncertainty/heuristic status, cost
units, and fallback when value is unresolved. Bounded heuristics may substitute
for calibrated expected value when identified and evaluated as such. Probability
and utility calibration claims follow EPI-CTRL-01.

The controller may deepen a trajectory, compare an alternative action, or stop
when further eligible simulation has insufficient value. Repeated descendants
of the same evidence/generation lineage retain their shared origin. Simulated
trajectories support model-relative comparisons; independent observations are
required to confirm external events. Exhausted budgets use the existing
unresolved-uncertainty or completion state.
