---
id: GONI-SPEC-DC17EFB4EB31
title: 2.3 Operator contract
type: specification
status: draft
implementation_state: specified_only
proposition: For delegable work, the runtime should infer missing structure and prospective work from policy, prior context, observed state, task class, and workflow evidence before interrupting the principal, while preserving uncertainty and compiling any consequential execution through an auditable WorkOrder and independent authority check.
domains:
- specs
aliases: []
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
- type: depends_on
  target: GONI-SPEC-F37FC6D98E05
sources:
- SRC-HORVITZ1999-MIXED-INITIATIVE
- SRC-KAUTZ1986-PLAN-RECOGNITION
artifacts: []
uncertainty: The mixed-initiative contract is specified only; prospective-task inference thresholds require evaluation.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 2.3 Operator contract
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 2.3 Operator contract

> Status boundary: this is a draft specification. Present-tense or enforcement
> language states intended contract behavior, not observed implementation,
> verification, or non-bypassability.

For delegable work, the runtime should:

- infer missing structure from policy, prior context, task class, and observed
  state before interrupting the principal,
- detect when observations plausibly indicate prospective work and represent it
  as a hypothesis rather than an instruction,
- preserve competing objective hypotheses when the distinction materially
  changes risk, corridor, tool choice, or irreversible side effects,
- classify whether current interaction is delegation or `co_creation` before
  tool planning,
- ask a clarification question when the answer materially changes risk,
  authority, tool choice, or irreversible side effects,
- surface assumptions and uncertainty when proceeding without clarification,
- compile explicit, triggered, or anticipated work into a WorkOrder before any
  consequential mutating call,
- keep prediction confidence independent from authority state.

This treats delegation as mixed-initiative control and plan recognition under
uncertainty rather than literal prompt completion.
