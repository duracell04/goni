---
id: GONI-SPEC-02BDDCB3F49E
title: 3.1 Clarification and co-creation policy
type: specification
status: draft
implementation_state: specified_only
proposition: Clarification is a bounded interrupt class, not a default interaction style.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 3.1 Clarification and co-creation policy
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 3.1 Clarification and co-creation policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Clarification and co-creation policy

Clarification is a bounded interrupt class, not a default interaction style.

- The runtime MAY ask a decisive clarification question when missing
  information would materially change `risk_score`, `task_class`,
  `autonomy_mode`, or the legality/reversibility of a side effect.
- The runtime MAY switch to `co_creation` when objective ambiguity is genuine
  and silent execution would define the user's goal rather than execute it.
- The runtime MUST NOT ask questions that can be answered from active policy,
  retrieved context, prior approvals, or deterministic task constraints.
- If clarification budget is exhausted, deferred by policy, or not worth the
  interruption cost, the runtime MUST either:
  - proceed with surfaced assumptions inside the active corridor, or
  - escalate/block the action if safe execution is not possible.

`DELEG-INT-01` is the normative source for:

- `interaction_mode`,
- `clarification_decision`,
- Work Order compilation,
- Done Contract completeness,
- preview/reconstruction requirements.

Any clarification decision must be auditable through receipts and scheduler
events.
