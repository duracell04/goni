---
id: GONI-SPEC-8386B8F4AE36
title: 5. Storage and reference contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'The control plane MUST preserve stable references for the pre-execution object: work_order_id interaction_mode work_quality_mode done_contract_hash clarification_decision objective_option_count For audit_grade work, it MUST also preserve: evidence_scope_ref search_strategy_ref claim_strength missing_evidence_ref audit_sticky'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 5. Storage and reference contract
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 5. Storage and reference contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Storage and reference contract

The control plane MUST preserve stable references for the pre-execution object:

- `work_order_id`
- `interaction_mode`
- `work_quality_mode`
- `done_contract_hash`
- `clarification_decision`
- `objective_option_count`

For `audit_grade` work, it MUST also preserve:

- `evidence_scope_ref`
- `search_strategy_ref`
- `claim_strength`
- `missing_evidence_ref`
- `audit_sticky`

Mutating actions with a compensation path SHOULD also preserve:

- `undo_strategy_ref`

These references may be stored in summarized form, but they must remain
replayable and auditable across tool calls, receipts, and review flows.
