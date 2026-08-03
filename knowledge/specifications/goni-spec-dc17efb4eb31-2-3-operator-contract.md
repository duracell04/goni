---
id: GONI-SPEC-DC17EFB4EB31
title: 2.3 Operator contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'For delegable work, the runtime MUST follow this operator contract: infer missing structure from policy, prior context, and task class before interrupting the user, classify whether the turn is delegation or co_creation before tool planning, ask a clarification question only when the answer materially changes risk, corridor, tool choice, or irreversible side effects,'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 2.3 Operator contract
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 2.3 Operator contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.3 Operator contract

For delegable work, the runtime MUST follow this operator contract:

- infer missing structure from policy, prior context, and task class before
  interrupting the user,
- classify whether the turn is `delegation` or `co_creation` before tool
  planning,
- ask a clarification question only when the answer materially changes risk,
  corridor, tool choice, or irreversible side effects,
- surface at most two candidate objectives when the goal itself is genuinely
  ambiguous,
- surface assumptions and uncertainty when proceeding without clarification,
- convert repaired intent into a Work Order, bounded plan, and explicit tool
  intent before any mutating call.

This treats delegation as mixed-initiative control under uncertainty rather
than as literal prompt completion [[horvitz1999-mixed-initiative]]
[[tomasev2026-intelligent-delegation]].
