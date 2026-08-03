---
id: GONI-SPEC-27DB21C47D8A
title: 1. Purpose
type: specification
status: draft
implementation_state: specified_only
proposition: Embodied AI is a delegation problem before it is a robotics problem.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 1. Purpose
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose

Embodied AI is a delegation problem before it is a robotics problem. A robot
that can see, move, touch, carry, unlock, clean, inspect, or assist people is
performing physical-world action under somebody's authority.

ROBOT-01 defines:

- the control boundary between local personal AI and robot execution,
- `RobotMandate`, the principal's bounded authorization for robot work,
- `EnvironmentScope`, the physical zone and social context in which a robot may
  act,
- `PhysicalActuationGrant`, the capability grant for movement and manipulation,
- `RobotAdapter`, the replaceable vendor or runtime bridge,
- `RobotReceiptBasis`, the receipt basis for physical-world action,
- task classes, cloud policy, safety posture, and conformance tests for
  embodied robot use.

The strategic claim behind this contract is that robot hardware may become
commoditized while private context, household memory, local policy, and trust
remain scarce. Goni therefore positions itself as the local command and
governance layer for embodied AI, not as the robot body.
