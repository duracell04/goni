---
id: GONI-SPEC-6A5B13D408B8
title: 7. Cloud Policy
type: specification
status: draft
implementation_state: specified_only
proposition: Cloud reasoning, vendor telemetry, robot fleet learning, diagnostics, remote operator access, and external model calls are egress.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 7. Cloud Policy
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 7. Cloud Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Cloud Policy

Cloud reasoning, vendor telemetry, robot fleet learning, diagnostics, remote
operator access, and external model calls are egress. They require explicit
authorization through [NET-01](/blueprint/30-specs/network-gate-and-anonymity.md)
and must preserve payload class, destination, purpose, redaction mode, budget,
and receipt metadata.

The default posture is local-first:

- private household memory is not sent to robot vendors by default,
- raw video, raw audio, private maps, and full sensor logs are denied for cloud
  upload unless policy explicitly permits the payload class,
- cloud planning may receive bounded task summaries and redacted refs only when
  the egress grant permits them,
- fleet learning may use compact, de-identified, policy-approved summaries only
  when the principal grants that use,
- remote supervision may not expand the original mandate or introduce new
  actuation powers,
- vendor clouds and remote operators cannot bypass Goni receipts.

If cloud reasoning is unavailable or denied, the robot action must either run
locally within its mandate, ask for approval, degrade to observe-only or
proposal mode, or block.
