---
id: GONI-IMAP-29C0128BED03
title: Receipts (specified only)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: receipt_id = row_id Fields: timestamp: timestamp(ms), trace_id: fixed_size_binary[16], span_id: fixed_size_binary[16], action_type: utf8, task_class: dict<uint8, utf8>, interaction_mode?: dict<uint8, utf8>, autonomy_mode: dict<uint8, utf8>, policy_decision: utf8, decision_basis: map<utf8, utf8>, risk_score: float32, risk_basis: map<utf8, utf8>, work_order_id?: fixed_size_binary[16], done_contract_hash?: fixed_size_binary[32], clarification_decision?: dict<uint8, utf8>, objective_option_count'
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
  heading: Receipts (specified only)
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Receipts (specified only)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Receipts (specified only)
- PK: `receipt_id = row_id`
- Fields: `timestamp: timestamp(ms)`, `trace_id: fixed_size_binary[16]`, `span_id: fixed_size_binary[16]`, `action_type: utf8`, `task_class: dict<uint8, utf8>`, `interaction_mode?: dict<uint8, utf8>`, `autonomy_mode: dict<uint8, utf8>`, `policy_decision: utf8`, `decision_basis: map<utf8, utf8>`, `risk_score: float32`, `risk_basis: map<utf8, utf8>`, `work_order_id?: fixed_size_binary[16]`, `done_contract_hash?: fixed_size_binary[32]`, `clarification_decision?: dict<uint8, utf8>`, `objective_option_count?: uint8`, `capability_id?: fixed_size_binary[16]`, `input_hash?: fixed_size_binary[32]`, `output_hash?: fixed_size_binary[32]`, `memory_read_refs: list<fixed_size_binary[16]>`, `memory_diff_refs?: list<fixed_size_binary[16]>`, `retrieval_basis?: map<utf8, utf8>`, `learning_basis?: map<utf8, utf8>`, `visual_basis?: map<utf8, utf8>`, `assumptions?: list<utf8>`, `uncertainty_level?: dict<uint8, utf8>`, `question_strategy?: dict<uint8, utf8>`, `tool_intent?: utf8`, `delegation_outcome?: dict<uint8, utf8>`, `undo_strategy_ref?: utf8`, `prev_hash?: fixed_size_binary[32]`, `chain_hash: fixed_size_binary[32]`
- Notes: minimal by default; no raw content. Delegated actions must carry surfaced assumptions and delegated-action outcome metadata.
