---
id: GONI-SPEC-70C934F7D55A
title: 4.4 Audit-grade work rules
type: specification
status: draft
implementation_state: specified_only
proposition: 'For audit_grade work, the runtime MUST apply EPISTATE-01 and preserve scope, evidence, inference, missing evidence, and negative-claim burden; absence of evidence in scope S is not evidence of absence outside scope S.'
domains:
- specs
aliases: []
relations:
- type: depends_on
  target: EPISTATE-01
sources: []
artifacts: []
uncertainty: The epistemic rules are specified architecture. Coverage thresholds and domain-specific negative-claim burdens require evaluation.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 4.4 Audit-grade work rules
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 4.4 Audit-grade work rules

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

### 4.4 Audit-grade work rules

For `audit_grade` work, the runtime MUST apply EPISTATE-01 and follow these
additional epistemic rules:

- **Absence-of-evidence rule:** absence of evidence in scope `S` is not evidence
  of absence outside scope `S`.
- **Scope declaration:** the Work Order must declare the planned or checked
  scope before strong conclusions are made.
- **Evidence before inference:** observed artifacts and derived conclusions must
  remain separable in receipts and user-facing summaries.
- **Null and missing-state discipline:** null, omitted, unavailable, or unchecked
  fields remain unknown or unspecified unless separate evidence supports a
  stronger proposition.
- **Negative-claim burden:** negative claims require stronger coverage than
  positive claims.
- **Missing-evidence surfacing:** if the scope is incomplete, the runtime must
  preserve what is missing and what next check would close the loop.
- **Sticky audit mode:** audit-grade mode persists for follow-up turns in the
  same task/session unless explicitly reset or a clear unrelated task boundary
  is detected and surfaced.

Audit-grade work may use richer evidence-strength labels, but it must not
collapse observed, inferred, hypothesized, verified, and certified state where
that distinction affects the conclusion.
