---
id: GONI-SPEC-113F3C00EC81
title: 5.3 PhysicalActuationGrant
type: specification
status: draft
implementation_state: specified_only
proposition: PhysicalActuationGrant is the capability grant for robot movement and manipulation.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 5.3 PhysicalActuationGrant
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 5.3 PhysicalActuationGrant

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.3 PhysicalActuationGrant

`PhysicalActuationGrant` is the capability grant for robot movement and
manipulation.

Minimum logical fields:

```yaml
physical_actuation_grant:
  grant_ref:
  robot_ref:
  allowed_motion_classes:
  allowed_manipulation_classes:
  allowed_task_classes:
  payload_limits:
  contact_limits:
  speed_or_force_limits:
  tool_or_end_effector_limits:
  device_operation_rules:
  person_interaction_rules:
  irreversible_action_rules:
  approval_requirement:
  emergency_stop_requirement:
  rollback_or_repair_ref:
  idempotency_rule:
  policy_hash:
  receipt_ref:
```

Physical actuation is default-deny. Movement, grasping, carrying, device
operation, door or lock interaction, cleaning, disposal, person assistance,
and security patrol are distinct action classes and may require distinct
approval gates.
