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
- clarification efficiency: vague prompts reach correct corridor/action within a
  bounded number of decisive questions
- question value: asked questions materially change corridor, risk, or tool
  choice often enough to justify interruption
- receipts and audit traces agree on corridor, `delegation_outcome`, and
  `question_strategy`

## Benchmark shape
- replay vague-intent traces under multiple delegation-policy bundles
- compare autonomous, review, blocked, and escalated outcomes against labels
- record turns, clarification count, receipt completeness, and final success

## Artifact links
- TBD
