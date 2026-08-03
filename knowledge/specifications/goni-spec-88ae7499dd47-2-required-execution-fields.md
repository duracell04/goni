---
id: GONI-SPEC-88AE7499DD47
title: 2. Required execution fields
type: specification
status: draft
implementation_state: specified_only
proposition: 'Receipts for metered executions MUST include: execution_id execution_type (action|tool|model) metering object with relevant counters budget_spent object (budget unit -> amount)'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-02-receipt-metering-fields.md
  heading: 2. Required execution fields
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2. Required execution fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Required execution fields
Receipts for metered executions MUST include:
- `execution_id`
- `execution_type` (`action|tool|model`)
- `metering` object with relevant counters
- `budget_spent` object (budget unit -> amount)
