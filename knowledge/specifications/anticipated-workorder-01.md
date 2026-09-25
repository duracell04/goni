---
id: ANTICIPATED-WORKORDER-01
title: Prospective and anticipated WorkOrder contract
type: specification
status: draft
implementation_state: specified_only
proposition: A prospective WorkOrder must identify whether its origin is explicit, triggered, or anticipated; anticipated work must preserve its prediction basis, confidence, competing objective hypotheses when material, and the independent authority basis required before consequential execution.
domains:
- specs
- delegation
- control
aliases:
- prospective-workorder
- anticipated-workorder
relations:
- type: refines
  target: GONI-SPEC-F37FC6D98E05
  note: Extends the canonical WorkOrder with origin and anticipation provenance.
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
sources:
- SRC-KAUTZ1986-PLAN-RECOGNITION
- SRC-CERAVOLO2024-PREDICTIVE-PROCESS-MONITORING
artifacts: []
uncertainty: Origin fields and hypothesis representation are specified only; calibration thresholds and storage representation require empirical validation.
legacy: []
---

# Prospective and anticipated WorkOrder contract

A WorkOrder may originate in three ways:

- `explicit`: the principal directly requested the work;
- `triggered`: a pre-existing mandate or rule activated on an observed event;
- `anticipated`: Goni inferred that the work is probably needed from current
  state and evidence.

An anticipated WorkOrder is a hypothesis-bearing pre-execution object. It does
not carry additional authority merely because prediction confidence is high.

The logical WorkOrder must preserve, directly or by stable references:

```yaml
origin:
  kind: explicit | triggered | anticipated
  trigger_refs: []
  inferred_at: timestamp

anticipation:
  prediction_confidence: 0.0..1.0
  prediction_basis:
    - prior_trajectory
    - active_objective
    - procedural_memory
    - domain_prior
  objective_hypotheses:
    - objective_ref: ...
      confidence: 0.0..1.0
  workflow_template_ref: optional
  predicted_next_steps: []

authority:
  mandate_ref: optional
  corridor_ref: optional
  capability_refs: []
  authority_basis_ref: optional
```

When multiple materially different objectives remain plausible, the WorkOrder
must preserve competing hypotheses until evidence, policy, or principal input
discriminates between them. Prediction confidence and authority state remain
separate fields.
