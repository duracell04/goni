---
id: GONI-SPEC-69FC23EE6DDE
title: Delegation fields (required for delegated or tool-mediated actions)
type: specification
status: draft
implementation_state: specified_only
proposition: 'task_class autonomy_mode risk_score risk_basis interaction_mode work_order_id done_contract_hash clarification_decision objective_option_count delegation The delegation object MUST expose stable delegation-engineering fields: assumptions uncertainty_level question_strategy tool_intent delegation_outcome undo_strategy_ref'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Delegation fields (required for delegated or tool-mediated actions)
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Delegation fields (required for delegated or tool-mediated actions)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Delegation fields (required for delegated or tool-mediated actions)
- task_class
- autonomy_mode
- risk_score
- risk_basis
- interaction_mode
- work_order_id
- done_contract_hash
- clarification_decision
- objective_option_count
- delegation

The `delegation` object MUST expose stable delegation-engineering fields:

- `assumptions`
- `uncertainty_level`
- `question_strategy`
- `tool_intent`
- `delegation_outcome`
- `undo_strategy_ref`
