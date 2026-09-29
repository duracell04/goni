---
id: GONI-IMAP-FDD10904EC6B
title: WorkOrders
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: work_order_id = row_id Fields include request and interaction identity, goal and completion contract, inputs and constraints, temporal_constraint_refs, assumptions, plan and tools, risk, clarification state, policy/state references, and provenance.'
domains:
- data
- software
aliases: []
relations:
- type: depends_on
  target: TIME-01
  note: Work Orders preserve stable references to material temporal constraints so delegated decisions remain reconstructable against deadline authority.
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength. The temporal_constraint_refs field is a specified-only refinement added after migration and does not claim an implemented table migration.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: WorkOrders
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# WorkOrders

> Status boundary: this is a migrated draft with a later specified-only refinement. Present-tense or enforcement language states intended contract behavior, not observed implementation, verification, or non-bypassability.

### WorkOrders
- PK: `work_order_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `interaction_mode: dict<uint8, utf8>`, `goal_summary: utf8`, `done_contract_hash: fixed_size_binary[32]`, `done_contract_summary: utf8`, `input_refs: list<utf8>`, `constraint_summary: utf8`, `temporal_constraint_refs: list<fixed_size_binary[16]>`, `assumption_refs: list<utf8>`, `plan_summary: utf8`, `tools: list<utf8>`, `risk_class: dict<uint8, utf8>`, `output_schema_ref?: utf8`, `clarification_decision: dict<uint8, utf8>`, `objective_option_count: uint8`, `created_at: timestamp(ms)`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: Canonical storage for pre-execution reconstruction. Raw doctrine text, raw prompts, and unbounded prose do not live here; use summaries, hashes, and references only.

### Temporal reconstruction

`constraint_summary` remains the compact human-readable summary of relevant constraints.

`temporal_constraint_refs` preserves stable machine-readable references to TIME-01 temporal constraints that materially govern the Work Order. A Work Order need not copy the underlying deadline semantics. It references the canonical temporal objects so policy, scheduler, tools, receipts, and later review can reconstruct:

- whether a date was an operative deadline or internal target;
- who had authority to change it;
- which change mechanism applied;
- which source and provenance supported the classification;
- whether a target protected another operative constraint.

A temporal reference is required when changing, negotiating, protecting, prioritizing around, or escalating because of a date materially affects delegated execution.

The presence of a temporal constraint does not itself grant authority to modify it. The applicable corridor, capability, policy, and TIME-01 authority semantics remain controlling.
