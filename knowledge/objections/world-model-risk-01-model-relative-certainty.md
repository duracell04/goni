---
id: WORLD-MODEL-RISK-01
title: A precise forecast can remain a poor model of reality
type: objection
status: draft
implementation_state: specified_only
proposition: Exact execution and internally coherent probabilities leave decision
  quality unresolved when the world model, prior, observations, or utility assumptions
  are misspecified.
domains:
- cognition
- evaluation
aliases: []
relations:
- type: objects_to
  target: WORLD-01
- type: objects_to
  target: SIM-BUDGET-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
- SRC-CASSANDRA-POMDP-TUTORIAL
artifacts: []
uncertainty: The magnitude of model error, calibration drift, and planning benefit
  is task-specific and unmeasured for GONI.
legacy: []
---

# A precise forecast can remain a poor model of reality

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

WORLD-01's belief formalism can create unwarranted precision if an implementation
assigns arbitrary numerical confidences or treats imagined trajectories as
independent observations. A rich actor model can also consume more resources or
private context than a decision needs.

Finite task scopes, explicit approximation/calibration status, provenance, model
checks and matched outcome evaluation address this objection. Bounded qualitative
hypotheses are a permitted alternative when a calibrated posterior is unavailable.
Predictive accuracy and decision utility require separately scoped measurements.
