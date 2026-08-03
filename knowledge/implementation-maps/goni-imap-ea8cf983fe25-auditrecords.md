---
id: GONI-IMAP-EA8CF983FE25
title: AuditRecords
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: audit_id = row_id Fields: agent_id: fixed_size_binary[16], policy_hash: fixed_size_binary[32], state_snapshot_id: fixed_size_binary[16], capability_token_id: fixed_size_binary[16], tool_id: dict<uint8, utf8>, args_hash: fixed_size_binary[32], result_hash: fixed_size_binary[32], timestamp: timestamp(ms), provenance: map<utf8, utf8>, task_class: dict<uint8, utf8>, interaction_mode: dict<uint8, utf8>, autonomy_mode: dict<uint8, utf8>, risk_score: float32, risk_basis: map<utf8, utf8>, work_order'
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
  heading: AuditRecords
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# AuditRecords

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### AuditRecords
- PK: `audit_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `capability_token_id: fixed_size_binary[16]`, `tool_id: dict<uint8, utf8>`, `args_hash: fixed_size_binary[32]`, `result_hash: fixed_size_binary[32]`, `timestamp: timestamp(ms)`, `provenance: map<utf8, utf8>`, `task_class: dict<uint8, utf8>`, `interaction_mode: dict<uint8, utf8>`, `autonomy_mode: dict<uint8, utf8>`, `risk_score: float32`, `risk_basis: map<utf8, utf8>`, `work_order_id: fixed_size_binary[16]`, `intent_summary: utf8`, `plan_summary: utf8`, `done_contract_hash: fixed_size_binary[32]`, `tool_intent: utf8`, `clarification_decision: dict<uint8, utf8>`, `clarification_status: dict<uint8, utf8>`, `objective_option_count: uint8`, `delegation_outcome: dict<uint8, utf8>`, `undo_strategy_ref?: utf8`
- Notes: audit rows for mutating calls must preserve the visible `intent -> plan -> tool intent` chain without storing raw transcripts.
