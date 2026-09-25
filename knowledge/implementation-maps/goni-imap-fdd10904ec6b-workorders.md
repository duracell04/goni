---
id: GONI-IMAP-FDD10904EC6B
title: WorkOrders
type: implementation-map
status: draft
implementation_state: specified_only
proposition: WorkOrders remain the canonical pre-execution control-plane record and should preserve explicit, triggered, or anticipated origin, prediction provenance, independent authority basis, authorized-prefix decisions, and bounded references needed for reconstruction.
domains:
- data
- software
aliases: []
relations:
- type: depends_on
  target: ANTICIPATED-WORKORDER-01
- type: depends_on
  target: AUTHORIZED-PLAN-PREFIX-01
sources: []
artifacts: []
uncertainty: The logical fields are specified; the executable schema and migrations remain future implementation work.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: WorkOrders
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# WorkOrders

> Status boundary: this is a draft implementation map. It defines the intended
> canonical shape; it does not claim a shipping table or migration exists.

PK: `work_order_id = row_id`.

Core existing fields remain:

- `request_id`
- `interaction_mode`
- `goal_summary`
- `done_contract_hash`
- `done_contract_summary`
- `input_refs`
- `constraint_summary`
- `assumption_refs`
- `plan_summary`
- `tools`
- `risk_class`
- `output_schema_ref`
- `clarification_decision`
- `objective_option_count`
- `created_at`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

Anticipatory delegation adds bounded logical fields:

- `origin_kind: explicit | triggered | anticipated`
- `trigger_refs: list<ref>`
- `prediction_confidence?: float32`
- `prediction_basis?: map<utf8, utf8>`
- `objective_hypothesis_refs?: list<ref>`
- `workflow_template_ref?: ref`
- `predicted_next_step_refs?: list<ref>`
- `authority_basis?: map<utf8, utf8>`
- `authorized_prefix_len?: uint16`
- `commit_boundary_ref?: ref`

Raw prompts, raw doctrine text, unbounded trajectory text, and private source
content do not belong in the WorkOrders row. Use stable references, compact
summaries, hashes, and dedicated memory or receipt objects.

The executable Arrow/schema representation should be updated only when the
prototype implementation adopts these fields; this map intentionally preserves
the current repository's distinction between specified design and pinned
implementation evidence.
