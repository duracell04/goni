---
id: GONI-IMAP-FDD10904EC6B
title: WorkOrders
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: work_order_id = row_id Fields: request_id: fixed_size_binary[16], interaction_mode: dict<uint8, utf8>, goal_summary: utf8, done_contract_hash: fixed_size_binary[32], done_contract_summary: utf8, input_refs: list<utf8>, constraint_summary: utf8, assumption_refs: list<utf8>, plan_summary: utf8, tools: list<utf8>, risk_class: dict<uint8, utf8>, output_schema_ref?: utf8, clarification_decision: dict<uint8, utf8>, objective_option_count: uint8, created_at: timestamp(ms), policy_hash: fixed_size_b'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: WorkOrders
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# WorkOrders

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### WorkOrders
- PK: `work_order_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `interaction_mode: dict<uint8, utf8>`, `goal_summary: utf8`, `done_contract_hash: fixed_size_binary[32]`, `done_contract_summary: utf8`, `input_refs: list<utf8>`, `constraint_summary: utf8`, `assumption_refs: list<utf8>`, `plan_summary: utf8`, `tools: list<utf8>`, `risk_class: dict<uint8, utf8>`, `output_schema_ref?: utf8`, `clarification_decision: dict<uint8, utf8>`, `objective_option_count: uint8`, `created_at: timestamp(ms)`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: Canonical storage for pre-execution reconstruction. Raw doctrine text, raw prompts, and unbounded prose do not live here; use summaries, hashes, and references only.
