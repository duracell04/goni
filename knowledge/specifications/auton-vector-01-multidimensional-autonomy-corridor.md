---
id: AUTON-VECTOR-01
title: Multidimensional autonomy corridor
type: specification
status: draft
implementation_state: specified_only
proposition: Autonomy should be represented as function-specific delegated authority across observation, analysis, decision selection, execution, persistence, and subdelegation rather than as one global scalar or mode.
domains: [delegation, autonomy, hci, specs]
aliases:
- autonomy vector
- functional autonomy corridor
relations:
- type: refines
  target: AGENTICNESS-01
- type: depends_on
  target: DELEG-MODEL-01
- type: synthesizes
  target: EVID-AUTOMATION-TYPES-01
sources:
- SRC-PARASURAMAN2000-AUTOMATION-LEVELS
- SRC-HORVITZ1999-MIXED-INITIATIVE
artifacts: []
uncertainty: The dimensions are a Goni extension of classical automation taxonomies and require task-specific calibration; they are not asserted to be the uniquely correct decomposition.
legacy: []
---

# Multidimensional autonomy corridor

A single label such as `manual`, `assist`, or `autonomous` is too coarse for persistent delegated agents.

Goni SHOULD represent an autonomy corridor as a vector over distinct functions:

[
A=(a_o,a_a,a_d,a_e,a_p,a_s)
]

where:

- (a_o): **observation** — authority to acquire permitted information;
- (a_a): **analysis** — authority to infer, classify, summarize, and plan;
- (a_d): **decision selection** — authority to choose among materially different courses of action;
- (a_e): **execution** — authority to cause external or durable effects;
- (a_p): **persistence** — authority to create or modify durable memory, policy-relevant state, or recurring behavior;
- (a_s): **subdelegation** — authority to delegate bounded work or capabilities to another worker.

Each dimension can have its own scope, conditions, budget, approval threshold, and verification requirements.

Example:

[
A=(high,high,medium,low,low,none)
]

could permit autonomous observation and analysis, bounded recommendation selection, approval-gated execution and persistence, and zero subdelegation.

This model preserves mixed initiative while making the authority surface explicit enough for policy and receipt systems to reason about it.
