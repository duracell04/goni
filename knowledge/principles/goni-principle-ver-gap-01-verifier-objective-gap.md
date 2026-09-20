---
id: GONI-PRINCIPLE-VER-GAP-01
title: "Verifier–Objective Gap"
type: principle
status: draft
implementation_state: specified_only
proposition: "Passing a verifier establishes only the property represented by that verifier; it does not establish global task correctness or complete alignment with the principal's objective."
domains:
- evaluation
- governance
aliases: []
relations:
- type: refines
  target: GONI-PRINCIPLE-VER-EVID-01
- type: supports
  target: EVID-HARNESS-01
sources: []
artifacts: []
uncertainty: "The size of the verifier–objective gap is task-specific and must be investigated empirically."
legacy: []
---

# Verifier–Objective Gap

For a verifier (V):

[
V(x)=1 
otRightarrow x=globally correct
]

A test, judge, policy check, benchmark, or reward signal measures a proxy for the desired outcome. Increasing optimizer strength raises the importance of keeping that proxy aligned with the principal's actual objective.

Goni evaluation should combine visible tests with relevant invariants, adversarial cases, hidden or withheld checks where appropriate, and end-to-end outcome review. A passed verifier is evidence with a defined scope.
