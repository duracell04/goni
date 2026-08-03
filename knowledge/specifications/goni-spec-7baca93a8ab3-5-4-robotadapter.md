---
id: GONI-SPEC-7BACA93A8AB3
title: 5.4 RobotAdapter
type: specification
status: draft
implementation_state: specified_only
proposition: RobotAdapter is the replaceable bridge between Goni and a robot vendor, runtime, fleet manager, or middleware stack.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 5.4 RobotAdapter
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 5.4 RobotAdapter

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.4 RobotAdapter

`RobotAdapter` is the replaceable bridge between Goni and a robot vendor,
runtime, fleet manager, or middleware stack.

Minimum logical fields:

```yaml
robot_adapter:
  adapter_ref:
  vendor_robot_id:
  robot_model:
  firmware_or_runtime_ref:
  supported_task_classes:
  supported_skill_refs:
  sensor_surfaces:
  safety_capabilities:
  local_execution_capabilities:
  cloud_dependency_declaration:
  telemetry_surfaces:
  remote_operator_capabilities:
  audit_log_refs:
  attestation_refs:
  adapter_policy_hash:
  provenance:
```

The adapter declares capability and evidence surfaces. It does not define
authority. Vendor logs, robot-cloud audit trails, ROS bags, telemetry streams,
or teleoperation logs may support a Goni receipt, but they do not replace the
canonical receipt.
