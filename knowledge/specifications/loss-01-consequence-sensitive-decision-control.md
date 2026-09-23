---
id: LOSS-01
title: Consequence-Sensitive Decision Control
type: specification
status: draft
implementation_state: specified_only
proposition: "Autonomous acceptance, escalation, clarification, deferral, or denial should be selected using consequence-sensitive expected loss within the set of actions already permitted by kernel policy."
domains:
- evaluation
- policy
- routing
- specs
aliases:
- EXPECTED-LOSS-CONTROL
relations:
- type: depends_on
  target: CAL-01
- type: depends_on
  target: SELECTIVE-01
- type: refines
  target: GONI-SPEC-8911D8AA7EE0
- type: depends_on
  target: GONI-DECISION-9AF49466170D
sources:
- SRC-BERGER1985-STATISTICAL-DECISION-THEORY
artifacts: []
uncertainty: "Loss functions, consequence classes, and acceptable residual risk require domain-specific governance and cannot be inferred from model confidence alone."
legacy: []
---

# Consequence-Sensitive Decision Control

Let A be the available cognitive/control actions, such as accept, verify,
escalate, ask, defer, or deny. Let Y describe the uncertain relevant outcome
and let c denote consequence context.

Within the action set already permitted by policy, a decision rule may select:

a*(x,c) = argmin over a in A_allowed of E[L(a,Y,c) | x].

This separates three questions:

1. what outcomes appear likely;
2. what losses follow from each action/outcome pair;
3. which actions are permitted at all.

The third question is authoritative and remains kernel-owned.

## Asymmetric error costs

False acceptance and unnecessary escalation need not have equal cost. For some
effect classes:

L(false authorization) >> L(unnecessary escalation).

For reversible low-impact classification the gap may be much smaller.

Therefore one global confidence threshold is not an adequate policy for all
tasks. Operating points SHOULD be scoped by consequence class and validated
against the relevant loss structure.

## Hard constraints dominate optimization

Expected-loss optimization MUST operate inside policy constraints:

A_allowed = {a in A : PolicyAllows(a, S_authority)}.

If policy denies an effect, predictive confidence and expected utility cannot
authorize it.

Irreversible, legally constrained, privacy-sensitive, or otherwise
consequential actions MAY impose explicit approval requirements independent of
confidence.
