---
id: GONI-IMAP-B41EA5BDE602
title: Receipts table (specified only)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The receipts table is specified in blueprint/software/50-data/51-schemas-mvp.md and will be added to the schema DSL in a later revision.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/53-schema-dsl-and-macros.md
  heading: Receipts table (specified only)
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# Receipts table (specified only)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Receipts table (specified only)
The receipts table is specified in blueprint/software/50-data/51-schemas-mvp.md
and will be added to the schema DSL in a later revision. When implemented, it
must preserve the same delegation fields as the receipt JSON schema and the
receipts spec: `interaction_mode`, `work_order_id`, `done_contract_hash`,
`clarification_decision`, `objective_option_count`, surfaced assumptions,
uncertainty level, question strategy, tool intent, delegation outcome, and
`undo_strategy_ref`. It must also preserve governed memory and learning fields:
`memory_read_refs`, `memory_diff_refs`, `retrieval_basis`, and
`learning_basis`.
