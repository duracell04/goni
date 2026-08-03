---
id: GONI-SPEC-F92B0DCADA40
title: 3. Pre-execution pipeline
type: specification
status: draft
implementation_state: specified_only
proposition: 'Before any mutating tool proposal, externally visible action, or audit-grade conclusion, the runtime MUST execute the following logical pipeline: parse_intent classify_interaction_mode classify_work_quality_mode decide ask / assume / propose / block compile_work_order execute_under_corridor_policy emit_receipt The pipeline is ordered.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 3. Pre-execution pipeline
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 3. Pre-execution pipeline

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Pre-execution pipeline

Before any mutating tool proposal, externally visible action, or audit-grade
conclusion, the runtime MUST execute the following logical pipeline:

1. `parse_intent`
2. `classify_interaction_mode`
3. `classify_work_quality_mode`
4. `decide ask / assume / propose / block`
5. `compile_work_order`
6. `execute_under_corridor_policy`
7. `emit_receipt`

The pipeline is ordered. A later stage may not silently redefine the goal,
Done Contract, scope, or claim-strength boundary without producing a new Work
Order reference and updated receipt metadata.
