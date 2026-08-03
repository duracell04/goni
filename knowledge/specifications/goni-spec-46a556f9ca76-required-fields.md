---
id: GONI-SPEC-46A556F9CA76
title: Required fields
type: specification
status: draft
implementation_state: specified_only
proposition: receipt_id timestamp trace_id span_id actor_id action_type capability_id task_class autonomy_mode policy_decision decision_basis risk_score risk_basis budget_delta input_hash output_hash memory_read_refs memory_diff_refs boundary_basis (required when observation, extraction, memory, actuation, sandbox, approval, or egress boundaries affected the decision)
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Required fields
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Required fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Required fields
- receipt_id
- timestamp
- trace_id
- span_id
- actor_id
- action_type
- capability_id
- task_class
- autonomy_mode
- policy_decision
- decision_basis
- risk_score
- risk_basis
- budget_delta
- input_hash
- output_hash
- memory_read_refs
- memory_diff_refs
- boundary_basis (required when observation, extraction, memory, actuation,
  sandbox, approval, or egress boundaries affected the decision)
