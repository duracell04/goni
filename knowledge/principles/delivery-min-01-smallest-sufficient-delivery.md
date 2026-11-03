---
id: DELIVERY-MIN-01
title: "Smallest sufficient software-delivery architecture"
type: principle
status: draft
implementation_state: specified_only
proposition: "Choose the least burdensome delivery architecture that satisfies declared product, security, recoverability, and assurance requirements."
domains:
- software
- validation
- delivery
aliases: []
relations:
- type: refines
  target: GONI-DECISION-635669877246
- type: depends_on
  target: GONI-DECISION-EF1D6F7F085E
sources:
- SRC-BARDSIRI2026-GHA-LANGUAGE
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "Normative engineering principle motivated by observational literature; an optimal CI/CD architecture for GONI has not been established empirically."
legacy: []
---

# Smallest sufficient software-delivery architecture

## Principle

GONI SHOULD maintain the least burdensome delivery system that satisfies its functional, security, authority, recoverability, and assurance requirements. Required guarantees are hard constraints; complexity is minimized only among conformant alternatives.

## Decision rule

Before introducing another pipeline stage, runner, orchestration service, configuration layer, or deployment target, document: (1) the requirement or failure it addresses; (2) why the simpler baseline is insufficient; (3) the test or evidence for its expected benefit; (4) additional maintenance, security, investigation, infrastructure, compute, network, and cognitive costs; and (5) the owner and removal or rollback condition.

Prefer one verified reference deployment and narrow, risk-linked checks. Maintain additional environments only where an independent need warrants their full testing and operational overhead. Review mature mechanisms for consolidation when their incremental value is not demonstrated.

## Boundary and evidence

This principle imposes no universal upper bound on workflow steps and does not select a CI vendor or deployment technology. Workflow-complexity studies report correlations, not proof that fewer steps cause greater reliability. Test alternatives against the same mandatory guarantees in EVAL-DELIVERY-01.
