---
id: GONI-SPEC-0A4D91952DF9
title: 5.5 RobotReceiptBasis
type: specification
status: draft
implementation_state: specified_only
proposition: RobotReceiptBasis is the robot-specific receipt basis attached to a REC-01 receipt when robot observation, extraction, memory, actuation, egress, or supervision affects an action.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 5.5 RobotReceiptBasis
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 5.5 RobotReceiptBasis

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.5 RobotReceiptBasis

`RobotReceiptBasis` is the robot-specific receipt basis attached to a REC-01
receipt when robot observation, extraction, memory, actuation, egress, or
supervision affects an action.

Minimum logical fields:

```yaml
robot_basis:
  robot_ref:
  adapter_ref:
  mandate_ref:
  environment_scope_refs:
  physical_actuation_grant_ref:
  task_class:
  skill_ref:
  sensor_basis_refs:
  map_or_zone_refs:
  policy_decision:
  safety_envelope_result:
  verification_result:
  intervention_state:
  escalation_or_denial_reason:
  egress_refs:
  remote_supervision_refs:
  output_or_state_delta_refs:
  rollback_or_repair_ref:
```

`robot_basis` stores compact refs, hashes, bounded summaries, and replay
metadata. It MUST NOT store raw private home maps, raw video, raw audio, full
sensor logs, unrestricted telemetry streams, or unbounded transcripts by
default.
