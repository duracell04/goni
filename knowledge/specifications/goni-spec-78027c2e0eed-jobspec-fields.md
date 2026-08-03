---
id: GONI-SPEC-78027C2E0EED
title: JobSpec fields
type: specification
status: draft
implementation_state: specified_only
proposition: 'job_id class: interactive | background | maintenance priority deadline (optional) cancel_policy budgets required_capabilities'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/job.md
  heading: JobSpec fields
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# JobSpec fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## JobSpec fields
- job_id
- class: interactive | background | maintenance
- priority
- deadline (optional)
- cancel_policy
- budgets
- required_capabilities
