---
id: GONI-SPEC-D0DECC14C1B8
title: 2.3 WorkQualityMode
type: specification
status: draft
implementation_state: specified_only
proposition: 'WorkQualityMode = best_effort | audit_grade best_effort: bounded, reversible, low-stakes work where an incomplete search may be acceptable if assumptions are surfaced.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.3 WorkQualityMode
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.3 WorkQualityMode

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.3 WorkQualityMode

`WorkQualityMode = best_effort | audit_grade`

- `best_effort`: bounded, reversible, low-stakes work where an incomplete search
  may be acceptable if assumptions are surfaced.
- `audit_grade`: conservative work mode for verification, compliance,
  contribution review, security, negative claims, or any task where a missing
  source could make the conclusion misleading.

Audit-grade is about the **work Goni performs**, not merely the final answer
style. It changes search breadth, evidence handling, claim strength, receipts,
and conformance expectations.
