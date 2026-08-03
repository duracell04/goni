---
id: GONI-PRINCIPLE-0F5A00F69914
title: 12. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: Robot actuation requires a Work Order, Done Contract, Robot Mandate, capability token, policy decision, and receipt path.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 12. Invariants
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 12. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 12. Invariants

- Robot actuation requires a Work Order, Done Contract, Robot Mandate,
  capability token, policy decision, and receipt path.
- Robot observation does not imply extraction, memory, cloud upload,
  supervision, or actuation.
- Physical actuation is default-deny.
- Private maps, raw video, raw audio, full sensor logs, and unrestricted
  telemetry are not stored in receipts by default.
- Vendor cloud, fleet learning, diagnostics upload, and remote supervision
  require explicit egress grants through NET-01.
- Remote supervisors cannot expand the principal's mandate.
- Movement into denied zones fails closed and remains auditable.
- People-facing and high-risk physical actions require stricter gates than
  object-only logistics actions.
- Emergency stop, revocation, expiry, or superseded mandate state prevents
  future robot actuation.
- Financial robot actions also satisfy DAT-01.
- Third-party robot logs cannot replace canonical Goni receipts.
