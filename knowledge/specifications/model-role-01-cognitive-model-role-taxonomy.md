---
id: MODEL-ROLE-01
title: Cognitive Model Role Taxonomy
type: specification
status: draft
implementation_state: specified_only
proposition: "Model bundles should advertise finite cognitive roles and evaluated task profiles so routing selects mechanisms by required function and evidence rather than by provider or model name."
domains:
- models
- routing
- specs
aliases:
- COGNITIVE-MODEL-ROLES
relations:
- type: refines
  target: MODEL-REG-01
- type: depends_on
  target: COG-GRAPH-01
- type: supports
  target: AGENT-MODEL-ORTHO-01
sources: []
artifacts: []
uncertainty: "The initial role vocabulary is deliberately finite and revisable. A model may support several roles, and role quality remains empirical."
legacy: []
---

# Cognitive Model Role Taxonomy

A cognitive graph should request a required function before selecting a concrete
model bundle.

Initial role vocabulary:

- embedding
- retriever
- reranker
- classifier
- decision
- router
- proposer
- generator
- reasoner
- critic
- verifier
- guard
- compressor
- synthesizer
- perception

A model bundle may advertise multiple roles.

Role advertisement is not evidence of competence. For every promoted role, the
registry should link to evaluation evidence covering the relevant task classes,
quality, calibration where applicable, latency, resource use, privacy/egress
constraints, and known failure modes.

A graph node requests a role plus constraints. The router resolves an eligible
approved mechanism using task fit, evidence, and policy.

Provider names and model brands are implementation details rather than
architectural roles.
