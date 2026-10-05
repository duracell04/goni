---
id: CAL-DRIFT-01
title: Calibration Drift and Revalidation Lifecycle
type: specification
status: draft
implementation_state: specified_only
proposition: "Calibration artifacts and confidence thresholds should be distribution-scoped, versioned, monitored for invalidating changes, and revalidated before promotion to a changed operating domain."
domains:
- evaluation
- learning
- models
- specs
aliases:
- RECALIBRATION-LIFECYCLE
relations:
- type: depends_on
  target: CAL-01
- type: depends_on
  target: CONFIDENCE-01
- type: refines
  target: GONI-PRINCIPLE-GOV-LOOP-01
sources:
- SRC-OVADIA2019-UNCERTAINTY-SHIFT
artifacts: []
uncertainty: "Reliable drift detection is itself an empirical problem. This contract defines governance responses to suspected or known shift rather than one universal detector."
legacy: []
---

# Calibration Drift and Revalidation Lifecycle

Calibration evidence has an operating scope. It SHOULD NOT be assumed to remain
valid after material changes to model, harness, decision schema, user/task
population, input source, or environmental conditions.

The governed lifecycle is:

observe outcomes
-> compare with calibration assumptions
-> detect or suspect shift
-> mark affected calibration evidence degraded or out-of-scope
-> recalibrate or re-evaluate
-> promote, reject, or rollback

When calibration evidence becomes invalid or materially uncertain, the safe
default is to reduce autonomous coverage, increase verification, or escalate
rather than silently broadening authority.

Recalibration changes statistical evidence and routing behavior. It MUST NOT
expand capabilities, mandates, or approval corridors.

Promoted calibration changes SHOULD be versioned, evidence-linked, reversible,
and visible in relevant routing or decision receipts.
