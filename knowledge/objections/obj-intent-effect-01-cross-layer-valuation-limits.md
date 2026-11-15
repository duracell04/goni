---
id: OBJ-INTENT-EFFECT-01
title: 'Limits of Cross-Layer Optimization and Marginal-Value Estimates'
type: objection
status: draft
implementation_state: not_applicable
proposition: 'Cross-plane optimization may introduce coordination, security, and maintenance costs exceeding its task-level benefits, while uncertainty about human utility and external effects makes marginal-value assessment difficult.'
domains:
- agent
- research
- system
aliases: []
relations:
- type: objects_to
  target: INTENT-EFFECT-01
sources:
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
- SRC-HORVITZ1999-MIXED-INITIATIVE
artifacts: []
uncertainty: 'These are plausible objections rather than measured GONI failures; their magnitude depends on workload, runtime, and evaluation method.'
legacy: []
---

# Limits of Cross-Layer Optimization and Marginal-Value Estimates

- **Bottleneck dominance:** accelerating nondominant internal stages can leave task-level latency nearly unchanged.
- **Integration costs:** additional coordination can increase coupling, observability requirements, maintenance liabilities, and failure surfaces.
- **Semantic gap:** authorized execution and a successful tool acknowledgment do not automatically establish that the external outcome matches the user's intention.
- **Valuation uncertainty:** the benefit of information gathering, risk mitigation, or verification may be difficult to estimate; mandatory constraints remain binding regardless of apparent short-run value.
- **Novelty:** capabilities, operating-system scheduling, and AI runtimes already possess extensive prior art; integrating them does not itself establish an original contribution.

Retain the existing GONI architecture as the baseline. EXP-INTENT-EFFECT-01 must compare measured outcomes against that baseline and record regressions alongside any gains.
