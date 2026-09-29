---
id: GONI-IMAP-TIME01
title: TemporalConstraints
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The Control Plane should represent real-world temporal constraints as first-class records whose date role, change authority, provenance, and linkage to protected targets remain distinct from scheduler-facing job deadlines.
domains:
- data
- software
- scheduling
aliases: []
relations:
- type: depends_on
  target: TIME-01
  note: Implements the data-shape implications of the temporal-authority specification without claiming runtime implementation.
sources: []
artifacts: []
uncertainty: This is a specified-only logical schema map. Field names and physical Arrow encodings may be refined before implementation, but the semantic separation defined by TIME-01 must be preserved.
legacy: []
---

# TemporalConstraints

> Status boundary: this implementation map is `specified_only`. It defines the logical fields needed by TIME-01 and does not claim that a runtime table or schema DSL entry exists yet.

## Logical table

### TemporalConstraints

- PK: `temporal_constraint_id = row_id`
- Plane: Control
- Intended fields:
  - `subject_ref: utf8`
  - `date_role: dict<uint8, utf8>`
  - `due_at: timestamp(ms)`
  - `control_authority: dict<uint8, utf8>`
  - `change_mechanism: dict<uint8, utf8>`
  - `derived_class: dict<uint8, utf8>`
  - `source_kind: dict<uint8, utf8>`
  - `consequence_kind: dict<uint8, utf8>`
  - `source_ref: utf8`
  - `classification_confidence: float32`
  - `classification_basis: map<utf8, utf8>`
  - `protects_constraint_ref?: fixed_size_binary[16]`
  - `supersedes_constraint_ref?: fixed_size_binary[16]`
  - `created_at: timestamp(ms)`
  - `policy_hash: fixed_size_binary[32]`
  - `provenance: map<utf8, utf8>`

## Semantics

`TemporalConstraints` stores real-world time constraints and planning targets.

It MUST remain distinct from scheduler-facing `JobSpec.deadline`. A scheduler job may be generated because a temporal constraint is approaching, but its internal service deadline is not the same semantic object as the user's university, tax, client, contractual, or personal target date.

`derived_class` is a presentation/helper field derived from `date_role`, `control_authority`, and `change_mechanism`. The underlying authority fields remain canonical when the derived label and source metadata are insufficient to reconstruct the decision.

## Target protection

A `target` may point to the operative deadline it protects through `protects_constraint_ref`.

The safety margin is derived from linked timestamps:

```text
safety_margin = operative_deadline.due_at - target.due_at
```

The interval SHOULD be calculated rather than stored as an independent source of truth unless later performance requirements justify a cache with explicit invalidation semantics.

## Unknown classification

When `date_role`, `control_authority`, or `change_mechanism` is not known with sufficient confidence, the record preserves `unknown` rather than defaulting to a movable target.

`classification_basis`, `source_ref`, and `provenance` preserve the evidence needed for later clarification, correction, or reclassification.
