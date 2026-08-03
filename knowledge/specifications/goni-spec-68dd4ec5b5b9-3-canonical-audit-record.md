---
id: GONI-SPEC-68DD4EC5B5B9
title: 3. Canonical audit record
type: specification
status: draft
implementation_state: specified_only
proposition: Audit records are written for every tool call and state mutation.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 3. Canonical audit record
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 3. Canonical audit record

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Canonical audit record

Audit records are written for every tool call and state mutation. Required
fields:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `capability_token_id`
- `tool_id`
- `args_hash`
- `result_hash`
- `provenance`
- `task_class`
- `interaction_mode`
- `autonomy_mode`
- `risk_score`
- `risk_basis`
- `work_order_id`
- `intent_summary`
- `plan_summary`
- `done_contract_hash`
- `tool_intent`
- `clarification_decision`
- `clarification_status`
- `objective_option_count`
- `delegation_outcome`
- `undo_strategy_ref`

See `blueprint/software/50-data/51-schemas-mvp.md` for `AuditRecords` and `CapabilityTokens`.
