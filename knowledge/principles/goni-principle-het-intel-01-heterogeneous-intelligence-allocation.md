---
id: GONI-PRINCIPLE-HET-INTEL-01
title: "Heterogeneous Intelligence Allocation"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should allocate operations to the mechanism with comparative advantage, using probabilistic models for semantic judgment and deterministic systems for exact state, arithmetic, schemas, permissions, policy decisions, and executable verification where those properties can be computed directly."
domains:
- models
- system
aliases: []
relations:
- type: supports
  target: GONI-THESIS-7BFB74017D50
- type: refines
  target: MODEL-REG-01
sources:
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "The boundary between probabilistic and deterministic work is task-dependent and can shift as specialized models and conventional algorithms improve."
legacy: []
---

# Heterogeneous Intelligence Allocation

Goni should allocate work by comparative advantage:

- semantic ambiguity and open-ended synthesis → suitable model,
- exact arithmetic → deterministic computation,
- structured lookup → database or index,
- constraint solving → solver,
- authority and permissions → kernel policy,
- schema and type validity → deterministic validator,
- executable properties → executable tests.

This reduces the amount of correctness that depends on stochastic model behavior while preserving model freedom where semantic judgment has value.
