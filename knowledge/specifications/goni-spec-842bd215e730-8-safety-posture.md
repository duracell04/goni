---
id: GONI-SPEC-842BD215E730
title: 8. Safety Posture
type: specification
status: draft
implementation_state: specified_only
proposition: Physical actuation is default-deny.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 8. Safety Posture
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 8. Safety Posture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Safety Posture

Physical actuation is default-deny. Goni MUST require stricter gates for:

- people-facing actions,
- medical, health, eldercare, childcare, accessibility, or physical assistance
  tasks,
- security patrol, monitoring, access control, doors, locks, windows, alarms,
  and private areas,
- stove, oven, appliance, machinery, vehicle, tool, electrical, chemical, heat,
  water, fire, or hazardous-material actions,
- irreversible property actions such as disposal, destruction, movement to
  inaccessible areas, or actions with no reliable repair path,
- financial, purchasing, delivery, contract, or payment-linked robot actions,
- actions around children, pets, guests, workers, bystanders, or sleeping or
  incapacitated people.

Required safety controls include:

- revocation and emergency stop paths,
- denied-zone fail-closed behavior,
- explicit approval for high-risk actions,
- local policy mediation before command execution,
- verification before memory update or task completion,
- receipt-linked incident, escalation, denial, or intervention status,
- bounded rollback, repair, compensation, or "no rollback available" metadata.

When required safety, approval, supervision, or receipt support is missing,
execution fails closed.
