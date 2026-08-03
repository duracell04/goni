---
id: GONI-PRINCIPLE-8A02EE507C55
title: 7. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**I1 - Work-order first:** mutating execution and audit-grade conclusions require a Work Order.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 7. Invariants
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 7. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Invariants

- **I1 - Work-order first:** mutating execution and audit-grade conclusions
  require a Work Order.
- **I2 - Done-contract completeness:** executable turns require a Done Contract.
- **I3 - No silent goal selection:** unresolved objective ambiguity must route
  to `co_creation` or `block`, not hidden defaulting.
- **I4 - One-question bound:** decisive clarification is bounded and
  policy-controlled.
- **I5 - Kernel-backed preview:** previews and approval panels must be
  derivable from kernel state.
- **I6 - Stable references:** receipts and audit records must preserve
  `interaction_mode`, `work_quality_mode`, `work_order_id`, and
  `done_contract_hash` for delegated actions.
- **I7 - Audit-grade work:** verification and compliance tasks must preserve
  scope, evidence, inference, missing-evidence, and claim-strength metadata.
- **I8 - No absence laundering:** the runtime must not turn "not found in checked
  scope" into "does not exist".
