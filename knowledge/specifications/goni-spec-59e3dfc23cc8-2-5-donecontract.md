---
id: GONI-SPEC-59E3DFC23CC8
title: 2.5 DoneContract
type: specification
status: draft
implementation_state: specified_only
proposition: 'A stable DoneContract defines the conditions for task completion; successful tool calls or committed transactions alone do not establish that those conditions have been met.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Legacy DoneContract fields are retained. The completion-assessment rule is specified only; runtime enforcement and empirical outcomes remain untested.
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

### Completion assessment (normative)

A Work Order MUST NOT be reported as complete solely because a tool returned
`ok` or a transaction reported `committed`. The runtime MUST evaluate the
bound Done Contract's `must_verify` and `stop_condition` against task-appropriate
observable outcomes and supporting evidence before declaring completion.

The task-level completion assessment MUST distinguish:

- `passed`: the required checks support the deliverable and stop condition.
- `failed`: at least one required check demonstrably fails.
- `indeterminate`: required evidence is missing, inaccessible, or inconclusive.

The assessment MUST preserve the `work_order_id`, unchanged
`done_contract_hash`, evaluated checks, and evidence or missing-evidence
references through the existing audit/receipt mechanisms. It is distinct from
per-call tool status and transaction outcome. Evidence references follow
`REC-01` data minimization and access policy.

Only `passed` permits a claim of completed work. A `failed` assessment MAY
trigger bounded correction within the active autonomy corridor, retry policy,
and resource budgets. An `indeterminate` assessment MUST remain unresolved and
be surfaced or escalated under active policy. Corrections MUST NOT silently
change the Done Contract or enlarge permissions; material objective or
acceptance-criterion changes require a new authorized contract version.
