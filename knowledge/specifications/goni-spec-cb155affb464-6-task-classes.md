---
id: GONI-SPEC-CB155AFFB464
title: 6. Task Classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'ROBOT-01 defines the following task classes: | Task class | Meaning | | robot.observe | Sense an environment without extracting durable facts or acting.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 6. Task Classes
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 6. Task Classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Task Classes

ROBOT-01 defines the following task classes:

| Task class | Meaning |
| --- | --- |
| `robot.observe` | Sense an environment without extracting durable facts or acting. |
| `robot.inspect` | Analyze a bounded object, zone, condition, hazard, inventory, or state. |
| `robot.navigate` | Move through allowed zones without manipulation. |
| `robot.fetch` | Locate, grasp, and bring a permitted object. |
| `robot.carry` | Transport a permitted object between allowed zones. |
| `robot.sort` | Classify and place permitted objects into allowed destinations. |
| `robot.clean` | Perform bounded cleaning or tidying tasks. |
| `robot.assist_person` | Physically assist or interact with a person under strict policy. |
| `robot.operate_device` | Operate switches, appliances, doors, locks, tools, controls, or machinery. |
| `robot.security_patrol` | Observe or inspect for safety/security anomalies under explicit privacy rules. |
| `robot.remote_supervised_action` | Execute a robot action with remote human or vendor supervision. |

Policy may add narrower task classes. Mixed tasks inherit the strictest
approval, safety, egress, and receipt requirements among their component
classes.
