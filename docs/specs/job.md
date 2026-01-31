# Job Contract (JOB-01)
DOC-ID: JOB-01

Status: specified only / roadmap

Defines JobSpec and JobState for scheduler-visible work.

## JobSpec fields
- job_id
- class: interactive | background | maintenance
- priority
- deadline (optional)
- cancel_policy
- budgets
- required_capabilities

## JobState
- queued | running | preempted | cancelled | completed

## Conformance tests
- invalid JobSpec must be rejected
- cancel policy honored at preemption points
