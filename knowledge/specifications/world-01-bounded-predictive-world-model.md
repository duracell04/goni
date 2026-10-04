---
id: WORLD-01
title: Bounded predictive world model
type: specification
status: draft
implementation_state: specified_only
proposition: GONI may maintain a bounded, task-scoped predictive representation of
  uncertain world, user, and actor state, preserving observation lineage, model assumptions,
  and the epistemic distinction of simulated trajectories.
domains:
- cognition
- planning
- memory
aliases: []
relations:
- type: refines
  target: DREAM-01
- type: depends_on
  target: EPI-CTRL-01
- type: depends_on
  target: EPI-SEP-01
- type: depends_on
  target: LOSS-01
- type: depends_on
  target: COMP-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
- SRC-CASSANDRA-POMDP-TUTORIAL
- SRC-HA2018-WORLD-MODELS
- SRC-HAFNER2023-DREAMERV3
artifacts: []
uncertainty: State variables, model validity, calibration, approximation, and forecast
  utility require domain- and horizon-specific evaluation.
legacy: []
---

# Bounded predictive world model

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Scope and state

The world model is a task-scoped cognitive representation over existing governed
context and MemoryEntries. It represents only variables relevant to the current
decision and a bounded forecast horizon. Durable hypotheses and derivatives use
the existing memory lifecycle; transient belief state uses EPI-CTRL-01's frame.

Let \(X_t\) be latent world/user/actor state, \(o_t\) an observation, \(a_t\) a
performed action, and \(\mathcal M\) a named model with prior, transition and
observation likelihood. A calibrated probabilistic profile is conceptually:

\[
b_t=\Pr(X_t\mid o_{0:t},a_{0:t-1},\mathcal M),\qquad
T_{\mathcal M}=\Pr(X_{t+1}\mid X_t,a_t,e_t).
\]

Here \(e_t\) denotes exogenous influences rather than received evidence.
The observation model is \(\Pr(o_{t+1}\mid X_{t+1},a_t,\mathcal M)\).
The computational starting state \(S\) in COMP-01 is separate from latent
\(X_t\): exact execution of a forecast does not establish that the forecast is
accurate or that its inputs describe reality.

## Representation and update contract

A conforming profile MUST declare the represented variables, scope, model
revision, prior/support assumptions, observation provenance, action history,
forecast horizon, and update policy. It MUST distinguish observed evidence,
derived hypotheses, model estimates, and simulated/counterfactual trajectories.
Source changes, contradictions, staleness and model mismatch remain visible to
the ordinary epistemic controller.

Default profiles may use finite competing hypotheses and EPI-CTRL-01 support
labels. A profile presenting numerical probabilities as calibrated posteriors
MUST cite evaluation evidence for its model and operating scope. An approximate
belief state MUST name its approximation and limitations. A model-generated
confidence number is identified as an estimate until that condition is met.

## Planning integration

WORLD-01 supplies bounded predictions to LOSS-01's decision rule. Conceptually,
within the kernel-permitted action set:

\[
a^*=\arg\max_{a\in A_{allowed}}
[\mathbb E_{b_t}(U(a))-\lambda Risk(a)-\mu Compute(a)
-\nu Irreversibility(a)].
\]

This is an explanatory utility profile of consequence-sensitive decision control.
The profile states its time horizon, utility/penalty units and weights, and
accounts for each cost once. Canonical permission constraints determine
\(A_{allowed}\). Predictive user or actor models inform hypotheses; current
mandates and preferences continue to come from governed authority/memory.

DREAM-01 provides offline trajectory generation. SIM-BUDGET-01 governs the
additional simulation allocation question. Forecast results return as candidate
evidence or hypotheses through the existing epistemic and memory boundaries.
