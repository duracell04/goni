---
id: GONI-SPEC-0D2E4E212CFE
title: 3. Control Boundary
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni separates robot powers into distinct grants: | Power | Meaning | Boundary question | | robot_observation | The robot may perceive sensor surfaces such as cameras, microphones, lidar, depth, force, touch, joint state, location, or environmental signals.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 3. Control Boundary
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 3. Control Boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Control Boundary

Goni separates robot powers into distinct grants:

| Power | Meaning | Boundary question |
| --- | --- | --- |
| `robot_observation` | The robot may perceive sensor surfaces such as cameras, microphones, lidar, depth, force, touch, joint state, location, or environmental signals. | What may the robot sense, where, when, and under whose presence constraints? |
| `robot_extraction` | The robot or local AI may parse observed data into objects, people, rooms, hazards, inventory, activities, plans, or summaries. | What facts may be derived, and may extraction use cloud services? |
| `robot_memory` | Extracted facts, maps, object locations, household routines, or task outcomes may be stored, indexed, reused, synced, or forgotten. | What memory class may be written, retained, reviewed, or synced? |
| `robot_actuation` | The robot may move, navigate, grasp, carry, manipulate, clean, operate devices, interact with people, or change physical state. | What physical side effects are allowed, under which mandate, zone, safety envelope, approval, and receipt? |
| `physical_egress` | Robot data, telemetry, sensor summaries, maps, incidents, or command streams may leave the local node. | Which destination, payload class, redaction mode, purpose, and budget are permitted? |
| `remote_supervision` | A remote human or vendor system may observe, advise, approve, or control part of the robot session. | What can the supervisor see or do, and can their access expand the original mandate? |
| `cloud_reasoning` | A remote model or vendor cloud may perform planning, perception, skill selection, diagnostics, or fleet learning. | Which task, payload class, memory class, destination, and receipt tier allow remote reasoning? |

Granting one power MUST NOT imply any other power. A robot that may observe a
room may not automatically extract private facts, store memory, upload video,
call a remote operator, or perform physical actuation.
