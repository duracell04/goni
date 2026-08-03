---
id: GONI-SPEC-57AAC6298BBC
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: turns with recoverable objectives must classify as delegation turns with genuine objective ambiguity must classify as co_creation audit, compliance, contribution-review, verification, and negative-claim tasks must classify as audit_grade ask_decisive must occur only when one answer materially changes plan, risk, tool choice, irreversibility, or audit scope
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: Conformance tests
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- turns with recoverable objectives must classify as `delegation`
- turns with genuine objective ambiguity must classify as `co_creation`
- audit, compliance, contribution-review, verification, and negative-claim
  tasks must classify as `audit_grade`
- `ask_decisive` must occur only when one answer materially changes plan, risk,
  tool choice, irreversibility, or audit scope
- `propose_objectives` must never emit more than two options by default
- execution that proceeds under `assume` must surface assumptions in receipts
- preview data must be reproducible from Work Order + policy state
- audit-grade Work Orders must include evidence scope, search strategy, missing
  evidence, and claim-strength metadata
- negative claims must be downgraded to "not found in checked scope" unless the
  Work Order proves adequate coverage
- sticky audit mode must persist across follow-up turns unless reset or a clear
  unrelated task boundary is surfaced
- visual task classes must preserve source asset refs, permission class,
  output target, verification requirements, and no raw private image content in
  Control-plane fields
