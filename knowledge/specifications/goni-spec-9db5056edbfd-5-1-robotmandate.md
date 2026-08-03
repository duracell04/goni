---
id: GONI-SPEC-9DB5056EDBFD
title: 5.1 RobotMandate
type: specification
status: draft
implementation_state: specified_only
proposition: RobotMandate is the formal authorization that defines what robot work may be attempted on behalf of the principal.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 5.1 RobotMandate
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 5.1 RobotMandate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 RobotMandate

`RobotMandate` is the formal authorization that defines what robot work may be
attempted on behalf of the principal.

Minimum logical fields:

```yaml
robot_mandate:
  mandate_ref:
  principal_ref:
  robot_ref:
  work_order_id:
  purpose:
  allowed_task_classes:
  allowed_environment_refs:
  autonomy_mode:
  approval_thresholds:
  supervision_requirement:
  safety_envelope_ref:
  egress_policy_ref:
  evidence_requirements:
  revocation_ref:
  policy_hash:
  valid_from:
  expires_at:
  receipt_requirement:
  provenance:
```

The mandate grants no ambient physical authority. It only permits the task
classes, environments, autonomy mode, and evidence posture explicitly defined
by policy. A robot adapter cannot broaden the mandate through vendor defaults,
remote operator procedures, or fleet policy.
