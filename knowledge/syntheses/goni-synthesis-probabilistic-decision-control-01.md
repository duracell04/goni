---
id: GONI-SYNTHESIS-PROBABILISTIC-DECISION-CONTROL-01
title: Governed Probabilistic Decision Control
type: synthesis
status: draft
implementation_state: not_applicable
proposition: "Goni should treat probabilistic cognition, calibration, selective prediction, consequence-sensitive control, deterministic authority, mediated action, verification, and recalibration as distinct but connected layers in one governed feedback system."
domains:
- agent
- evaluation
- kernel
- models
- system
aliases:
- PROBABILISTIC-DECISION-CONTROL-SYNTHESIS
relations:
- type: synthesizes
  target: GONI-PRINCIPLE-HET-INTEL-01
- type: synthesizes
  target: DECISION-MODEL-01
- type: synthesizes
  target: CAL-01
- type: synthesizes
  target: SELECTIVE-01
- type: synthesizes
  target: LOSS-01
- type: synthesizes
  target: CONFIDENCE-01
- type: synthesizes
  target: CAL-DRIFT-01
- type: synthesizes
  target: GONI-DECISION-9AF49466170D
- type: synthesizes
  target: GONI-PRINCIPLE-GOV-LOOP-01
- type: synthesizes
  target: GONI-PRINCIPLE-VALIDITY-CORRECTNESS-01
- type: synthesizes
  target: GONI-PRINCIPLE-TRAJ-ECON-01
- type: synthesizes
  target: GONI-EXPERIMENT-DECISION-LAYER-01
sources: []
artifacts: []
uncertainty: "This synthesis introduces no implementation claim. Thresholds, calibrated operating regions, loss functions, and preferred mechanisms remain empirical and consequence-specific."
legacy: []
---

# Governed Probabilistic Decision Control

The combined architecture separates the questions that agentic systems often
collapse into a single model call.

1. What appears true or useful?
   Heterogeneous cognition selects an appropriate deterministic or probabilistic
   mechanism.

2. How reliable is that inference?
   Calibration evidence determines whether reported confidence has an empirical
   probability interpretation in the current domain.

3. Should the system accept the cognitive result?
   Selective prediction chooses acceptance or abstention at an evaluated
   risk-coverage operating point.

4. What is the consequence of being wrong?
   Consequence-sensitive control evaluates asymmetric losses and escalation
   costs.

5. Is the resulting effect permitted?
   The kernel resolves authority from canonical mandates, capabilities, policy,
   approvals, budgets, and revocation state.

6. What happened after action?
   Mediated execution is observed, verified, receipted, and reconciled with
   canonical state.

7. Does the statistical evidence still hold?
   Outcomes feed calibration monitoring, drift detection, re-evaluation, and
   controlled promotion or rollback.

The resulting conceptual flow is:

state
-> heterogeneous inference
-> calibrated uncertainty
-> selective accept / abstain
-> consequence-sensitive control
-> canonical kernel authorization
-> mediated action
-> observation and verification
-> receipt and state update
-> calibration / routing evidence update

## Core distinctions

Capability is not authority.

Confidence is not authority.

Schema validity is not semantic correctness.

Calibration is not accuracy.

A model can become more capable without receiving additional permissions. A
model can be highly confident without having permission to act. An output can be
perfectly typed and semantically wrong. A model can be accurate on average while
its probabilities are unreliable.

## Allocation objective

The system-level optimization problem is multi-objective. Depending on the task
it may consider:

- semantic task loss;
- calibration quality;
- selective risk and coverage;
- consequence-weighted expected loss;
- latency;
- monetary and compute cost;
- local energy/thermal cost;
- privacy and egress exposure;
- verification burden;
- human interruption;
- trajectory-level economics.

These objectives operate inside hard authority, privacy, and safety constraints.
They are not permitted to trade away a policy prohibition.

## Architectural doctrine

Probabilistic cognition supplies beliefs and proposals.

Statistical control determines when those beliefs are sufficiently supported to
accept, abstain, verify, or escalate.

The kernel determines whether an effect is authorized.

Tools execute only through mediated capabilities.

Receipts and verification make consequences reconstructable.

Observed outcomes improve future calibration and routing evidence without
silently modifying authority.

In compact form:

Models and algorithms estimate.
Calibration quantifies evidential reliability.
Selective control accepts or escalates.
Loss models consequence.
The kernel authorizes.
Tools act.
Verification observes.
Receipts prove.
Evidence recalibrates.
