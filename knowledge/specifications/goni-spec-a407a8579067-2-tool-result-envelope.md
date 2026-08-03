---
id: GONI-SPEC-A407A8579067
title: 2. Tool result envelope
type: specification
status: draft
implementation_state: specified_only
proposition: 'Tool results MUST include: tool_id agent_id state_snapshot_id status (ok | error) result_hash provenance operation_id task_class interaction_mode autonomy_mode risk_score work_order_id done_contract_hash tool_intent clarification_decision clarification_status objective_option_count delegation_outcome undo_strategy_ref tx_outcome (committed | rolled_back | no_op)'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 2. Tool result envelope
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 2. Tool result envelope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Tool result envelope

Tool results MUST include:

- `tool_id`
- `agent_id`
- `state_snapshot_id`
- `status` (ok | error)
- `result_hash`
- `provenance`
- `operation_id`
- `task_class`
- `interaction_mode`
- `autonomy_mode`
- `risk_score`
- `work_order_id`
- `done_contract_hash`
- `tool_intent`
- `clarification_decision`
- `clarification_status`
- `objective_option_count`
- `delegation_outcome`
- `undo_strategy_ref`
- `tx_outcome` (`committed` | `rolled_back` | `no_op`)
- `commit_delta_id` (present when `tx_outcome = committed`)
