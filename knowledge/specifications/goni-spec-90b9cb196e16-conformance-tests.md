---
id: GONI-SPEC-90B9CB196E16
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: clarification interrupts must be suppressed when the answer is derivable from policy or retrieved context clarification interrupts must be raised when missing information materially changes corridor, risk, or irreversible side effects co-creation interrupts must be raised when goal ambiguity is genuine and must be suppressed for mere factual omission
domains:
- specs
aliases: []
relations:
- type: tests
  target: TIME-01
  note: Adds conformance obligations for authority-aware deadline and target handling.
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: Conformance tests
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- clarification interrupts must be suppressed when the answer is derivable from
  policy or retrieved context
- clarification interrupts must be raised when missing information materially
  changes corridor, risk, or irreversible side effects
- co-creation interrupts must be raised when goal ambiguity is genuine and must
  be suppressed for mere factual omission
- clarification budget exhaustion must lead to surfaced assumptions,
  escalation, or blocking rather than repeated questioning
- scheduler audit fields must record `interaction_mode`,
  `clarification_decision`, `clarification_status`, and
  `delegation_outcome`


### Temporal-authority conformance

Implementations claiming TIME-01 conformance must demonstrate that:

- changing a `target` never silently mutates its linked
  `operative_deadline`;
- a `fixed` deadline cannot be rescheduled through ordinary scheduling
  authority;
- changing a `committed` deadline requires the applicable renegotiation,
  coordination, approval, or delegated-authority path;
- an `unknown` temporal constraint is not silently treated as freely movable;
- scheduler-facing `JobSpec.deadline` remains semantically distinguishable
  from real-world temporal constraints;
- changing either side of a target-to-operative-deadline link recomputes the
  derived safety margin;
- deadline-driven Work Orders preserve the relevant
  `temporal_constraint_refs` when the date materially affects planning,
  escalation, negotiation, or execution;
- open-loop handling may reschedule a target within policy but cannot solve
  committed or fixed deadline risk by rewriting the protected date;
- a failed attempt to modify a temporal constraint beyond granted authority is
  blocked or escalated and remains reconstructable through the normal audit and
  receipt path.

Suggested negative fixtures should include:

1. an overloaded schedule where moving an internal target is permitted but the
   linked university submission deadline remains unchanged;
2. a client delivery date that requires renegotiation before mutation;
3. a statutory or institutional cutoff that remains protected while movable
   work is reprioritized around it;
4. an ambiguous email-derived date whose authority is `unknown` and therefore
   cannot be silently converted into a target.
