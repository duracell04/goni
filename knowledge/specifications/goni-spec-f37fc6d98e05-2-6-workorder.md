---
id: GONI-SPEC-F37FC6D98E05
title: 2.6 WorkOrder
type: specification
status: draft
implementation_state: specified_only
proposition: Every executable or prospectively prepared unit of delegated work MUST compile a WorkOrder that preserves goal, completion contract, inputs, constraints, assumptions, plan, tools, risk, work quality, origin, and the independent authority basis; anticipated WorkOrders additionally preserve prediction provenance and competing objective hypotheses when material.
domains:
- specs
aliases: []
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft and extended with specified-only anticipation fields; storage and calibration remain unimplemented.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.6 WorkOrder
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.6 WorkOrder

> Status boundary: this is a draft specification. Present-tense or enforcement
> language states intended contract behavior, not observed implementation,
> verification, or non-bypassability.

Every executable or prospectively prepared unit of delegated work MUST compile
a `WorkOrder` with:

- `goal`
- `done_contract`
- `inputs`
- `constraints`
- `assumptions`
- `plan`
- `tools`
- `risk_class`
- `output_schema`
- `work_quality_mode`
- `origin`
- `authority`

`origin.kind` distinguishes `explicit`, `triggered`, and `anticipated`.
The authority object records the independent basis under which execution may
occur. Prediction confidence is not an authority field.

For `anticipated` work, the WorkOrder MUST additionally preserve the
anticipation provenance defined by `ANTICIPATED-WORKORDER-01`, including
prediction basis, confidence, workflow-template reference when applicable,
predicted steps, and competing objective hypotheses when they could materially
change risk, tools, or effects.

For `audit_grade` work, the Work Order MUST additionally carry:

- `evidence_scope`
- `search_strategy`
- `negative_claim_policy`
- `claim_strength_target`
- `missing_evidence_plan`
- `audit_sticky`

The WorkOrder remains the canonical pre-execution object.
