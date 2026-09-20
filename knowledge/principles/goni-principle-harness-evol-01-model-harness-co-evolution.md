---
id: GONI-PRINCIPLE-HARNESS-EVOL-01
title: "Model–Harness Co-Evolution"
type: principle
status: draft
implementation_state: specified_only
proposition: "Harness mechanisms introduced to compensate for a model weakness should carry explicit revalidation criteria so they can be retained, simplified, or removed as model capabilities change."
domains:
- agent
- evaluation
- system
aliases: []
relations:
- type: refines
  target: GONI-SYNTHESIS-DC66DA308CAE
- type: refines
  target: SYS-03
sources:
- SRC-LIN2026-AGENTIC-HARNESS-ENGINEERING
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "The optimal revalidation cadence depends on model turnover, task criticality, and the cost of maintaining each harness mechanism."
legacy: []
---

# Model–Harness Co-Evolution

The boundary between model capability and harness responsibility changes over time.

Every substantial compensatory mechanism should preserve the observed weakness it addresses, predicted system benefit, evaluation signal, revalidation trigger, and removal or simplification condition.

When a model generation changes materially, Goni should rerun the original evaluation before carrying the workaround forward. This limits scaffold debt and keeps the harness at the smallest sufficient level of complexity.
