# Job Contract (JOB-01)
DOC-ID: JOB-01

Status: Specified only / roadmap

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

## Upstream
- [Tool capability API](./tool-capability-api.md)

## Downstream
- [Scheduler and interrupts](./scheduler-and-interrupts.md)
- [Orchestrator](../../software/30-components/orchestrator.md)

## Adjacent
- [Agent manifest](./agent-manifest.md)
- [System map](../00-system-map.md)

## Conformance tests
- invalid JobSpec must be rejected
- cancel policy honored at preemption points


