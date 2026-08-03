---
id: GONI-SPEC-CC8AD2F60128
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: 'mutating delegated actions must have an auditable chain: intent_summary -> plan_summary -> tool_intent clarification interrupts must occur only when a missing answer would change corridor, risk, or irreversible behavior objective ambiguity must enter co_creation or block, not silent defaulting actions that proceed without clarification must surface assumptions and'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: Conformance tests
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- mutating delegated actions must have an auditable chain:
  `intent_summary -> plan_summary -> tool_intent`
- clarification interrupts must occur only when a missing answer would change
  corridor, risk, or irreversible behavior
- objective ambiguity must enter `co_creation` or block, not silent defaulting
- actions that proceed without clarification must surface assumptions and
  uncertainty in receipts
- irreversible actions must require explicit approval or a declared two-phase
  commit path
- failure-replay suites must classify at least the documented delegation
  failure modes
