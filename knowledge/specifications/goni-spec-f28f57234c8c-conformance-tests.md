---
id: GONI-SPEC-F28F57234C8C
title: Conformance Tests
type: specification
status: draft
implementation_state: specified_only
proposition: Robot actuation cannot occur without Work Order, Done Contract, Robot Mandate, capability token, and receipt path.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: Conformance Tests
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# Conformance Tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance Tests

- Robot actuation cannot occur without Work Order, Done Contract, Robot
  Mandate, capability token, and receipt path.
- Observation does not imply extraction, memory, cloud upload, or actuation.
- Private home maps, raw video, raw audio, and full sensor logs are omitted
  from receipts by default.
- Vendor cloud or fleet-learning upload is denied without explicit egress and
  payload classification.
- Robot movement into denied zones fails closed and emits an auditable denial.
- People-facing or high-risk physical actions require stricter approval than
  object-only logistics actions.
- Remote supervision cannot expand the original principal mandate.
- Financial or purchasing actions performed by a robot also satisfy DAT-01.
- Emergency stop, revocation, or mandate expiry prevents future robot
  actuation.
- Third-party robot logs are insufficient as the only evidence for a mediated
  physical action.
