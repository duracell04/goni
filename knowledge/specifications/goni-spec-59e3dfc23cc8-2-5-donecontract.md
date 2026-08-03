---
id: GONI-SPEC-59E3DFC23CC8
title: 2.5 DoneContract
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every executable turn MUST have a DoneContract with: deliverable must_include must_verify stop_condition The Done Contract is the kernel-visible statement of what counts as finished.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.5 DoneContract
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.5 DoneContract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.5 DoneContract

Every executable turn MUST have a `DoneContract` with:

- `deliverable`
- `must_include`
- `must_verify`
- `stop_condition`

The Done Contract is the kernel-visible statement of what counts as finished.
It must be hashable, stable across retries, and compact enough to reference in
receipts and audit records.

For `audit_grade` work, the Done Contract MUST also identify:

- the minimum evidence scope required for the conclusion,
- the allowed strength of negative claims,
- and whether missing evidence must be surfaced before completion.
