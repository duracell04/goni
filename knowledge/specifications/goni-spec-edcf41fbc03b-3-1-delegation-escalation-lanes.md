---
id: GONI-SPEC-EDCF41FBC03B
title: 3.1 Delegation escalation lanes
type: specification
status: draft
implementation_state: specified_only
proposition: 'Delegated execution must route through explicit lanes: autonomous: executes within active corridor and risk threshold, review: deferred/batched review for soft-gate actions, blocked: denied by no-go policy or risk overflow, escalated: requires immediate user decision.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 3.1 Delegation escalation lanes
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 3.1 Delegation escalation lanes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3.1 Delegation escalation lanes

Delegated execution must route through explicit lanes:

- `autonomous`: executes within active corridor and risk threshold,
- `review`: deferred/batched review for soft-gate actions,
- `blocked`: denied by no-go policy or risk overflow,
- `escalated`: requires immediate user decision.

The scheduler is responsible for fairness and latency bounds across these lanes.
