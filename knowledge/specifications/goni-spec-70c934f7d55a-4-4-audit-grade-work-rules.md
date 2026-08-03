---
id: GONI-SPEC-70C934F7D55A
title: 4.4 Audit-grade work rules
type: specification
status: draft
implementation_state: specified_only
proposition: 'For audit_grade work, the runtime MUST follow these epistemic rules: **Absence-of-evidence rule:** absence of evidence in scope S is not evidence of absence outside scope S.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 4.4 Audit-grade work rules
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 4.4 Audit-grade work rules

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.4 Audit-grade work rules

For `audit_grade` work, the runtime MUST follow these epistemic rules:

- **Absence-of-evidence rule:** absence of evidence in scope `S` is not evidence
  of absence outside scope `S`.
- **Scope declaration:** the Work Order must declare the planned or checked
  scope before strong conclusions are made.
- **Evidence before inference:** observed artifacts and derived conclusions must
  remain separable in receipts and user-facing summaries.
- **Negative-claim burden:** negative claims require stronger coverage than
  positive claims.
- **Missing-evidence surfacing:** if the scope is incomplete, the runtime must
  preserve what is missing and what next check would close the loop.
- **Sticky audit mode:** audit-grade mode persists for follow-up turns in the
  same task/session unless explicitly reset or a clear unrelated task boundary
  is detected and surfaced.
