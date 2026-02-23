---
id: EVID-AUTON-01
type: EVIDENCE
status: specified_only
---
# EVID-AUTON-01 Autonomy Corridor Validation

Goal: verify that delegated actions respect corridor policy
(`no_go` / `soft_gate` / `autopilot`) per task class.

## Core checks
- autonomous commit requires explicit corridor policy
- no-go actions are blocked with auditable receipts
- soft-gate actions enter review lane as configured
- autopilot actions meet risk threshold constraints

## Artifact links
- TBD
