---
id: ROBOT-01
title: ROBOT-01 - Embodied Robot Control Plane
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: ROBOT-01 Status: Specified only / roadmap This spec defines the control-plane contract for embodied robots that observe, move, manipulate, inspect, assist, or act in the physical world under delegated authority from a principal.'
domains:
- specs
aliases:
- EMBODIED-ROBOT-CONTROL-PLANE
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: ROBOT-01 - Embodied Robot Control Plane
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# ROBOT-01 - Embodied Robot Control Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# ROBOT-01 - Embodied Robot Control Plane
DOC-ID: ROBOT-01
Status: Specified only / roadmap

This spec defines the control-plane contract for embodied robots that observe,
move, manipulate, inspect, assist, or act in the physical world under delegated
authority from a principal.

Goni treats robot bodies, vendor skills, sensor streams, robot clouds, and
remote operators as replaceable execution substrates. They do not own the
principal's mandate, private memory, local policy, approval thresholds, or
receipt semantics.

Core doctrine:

```text
Personal AI owns mandate, memory, policy, and judgment.
Robot vendors provide body, sensors, skills, and actuation.
Cloud AI is optional, gated, and receipt-linked.
```

ROBOT-01 is specified only. It does not add a shipping schema table, define a
robot hardware platform, require a humanoid robot, or require any specific
robot vendor API. Implementations may later map this contract onto ROS,
vendor SDKs, fleet managers, teleoperation systems, industrial robot cells,
home robots, mobile manipulators, or humanoid platforms, but the Goni
authority model remains independent of those adapters.
