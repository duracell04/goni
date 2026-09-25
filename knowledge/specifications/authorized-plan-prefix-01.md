---
id: AUTHORIZED-PLAN-PREFIX-01
title: Maximum authorized plan prefix
type: specification
status: draft
implementation_state: specified_only
proposition: For a predicted or planned action sequence, Goni should prepare or execute the longest prefix whose actions are currently authorized and policy-permitted, and stage, defer, ask, or escalate exactly at the first action whose commit boundary exceeds current authority or risk constraints.
domains:
- specs
- delegation
- control
aliases:
- maximum-authorized-prefix
- safe-plan-prefix
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
- type: depends_on
  target: ANTICIPATED-WORKORDER-01
sources:
- SRC-PARASURAMAN2000-LEVELS-AUTOMATION
- SRC-HORVITZ1999-MIXED-INITIATIVE
artifacts: []
uncertainty: The prefix rule is normative design intent; practical authorization predicates, reversibility classes, and preparation costs require conformance tests.
legacy: []
---

# Maximum authorized plan prefix

Let a candidate plan be:

[
\pi = (a_1, a_2, \ldots, a_n).
]

For current state (s), let (A_K(s)) denote actions permitted by the kernel's
current policy, mandate, capabilities, budgets, approval state, and risk
constraints.

Goni should select:

[
k^* = \max \{k : \forall i \le k, a_i \in A_K(s)\}.
]

The system may then prepare or execute ((a_1, \ldots, a_{k^*})) according to
the action-specific corridor and stop at the first unresolved commit boundary.

This rule is function-specific rather than one scalar autonomy level. A corridor
may, for example, allow observation, retrieval, analysis, prediction, and draft
preparation automatically while requiring review for external selection or
action implementation.

The boundary decision and its basis must be reconstructable in the WorkOrder and
receipt chain.
