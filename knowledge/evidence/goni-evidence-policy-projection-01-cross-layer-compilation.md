---
id: GONI-EVIDENCE-POLICY-PROJECTION-01
title: Declarative routing policy can compile into multiple orchestration and infrastructure targets
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Chen et al. (2026) describe a declarative routing policy compiled into model-routing, agent-orchestration, infrastructure, MCP, and A2A decision artifacts, supporting Goni's use of one canonical policy source with derived surface-specific projections."
domains:
- research
- policy
- routing
aliases: []
relations:
- type: supports
  target: POLICY-PROJECTION-01
- type: supports
  target: COG-GRAPH-01
sources:
- SRC-CHEN2026-ROUTING-AGENT-ORCHESTRATION
artifacts: []
uncertainty: "The paper's policy language, compiler guarantees, and deployment targets are specific to that system. Goni's stronger capability and kernel authority model remains independent and requires its own implementation proof."
legacy: []
---

# Declarative routing policy can compile into multiple targets

Chen et al. extend a declarative routing DSL from stateless model selection to
multi-step agent workflows and infrastructure/protocol controls.

The reported compiler emits decision structures or artifacts for orchestration
frameworks, Kubernetes/network controls, MCP, and A2A boundaries from a common
policy source.

This provides external support for the POLICY-PROJECTION-01 design intuition:
cross-layer policy consistency can be improved by deriving surface-specific
eligibility from one source rather than maintaining unrelated copies.

Goni does not adopt the source system's DSL or treat compiled routing policy as
canonical authority. SPEC-POL-01 and kernel capability state remain the
normative authority source; projections may only preserve or narrow it.
