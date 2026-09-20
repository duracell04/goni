---
id: GONI-PRINCIPLE-TRAJ-EVAL-01
title: "Trajectory-Level Agent Evaluation"
type: principle
status: draft
implementation_state: specified_only
proposition: "Agent evaluation should measure the quality of the execution trajectory in addition to terminal task success, including recovery, unnecessary actions, constraint violations, latency, cost, state integrity, and reproducibility."
domains:
- agent
- evaluation
aliases: []
relations:
- type: refines
  target: GONI-SPEC-5211A9E877AD
- type: refines
  target: GONI-LAB
- type: supports
  target: EVID-HARNESS-01
sources: []
artifacts: []
uncertainty: "Metric weights must vary by task class and consequence; no single scalar score is specified here."
legacy: []
---

# Trajectory-Level Agent Evaluation

Terminal success can hide fragile execution. Goni should evaluate both the final outcome and how the system reached it.

```text
E(trajectory) =
(success,
 recovery,
 unnecessary actions,
 violations,
 latency,
 cost,
 state integrity,
 reproducibility)
```

Metrics should remain decomposed rather than collapsing immediately into one score. This preserves the evidence needed to distinguish efficient, policy-compliant execution from accidental or excessively costly success.
