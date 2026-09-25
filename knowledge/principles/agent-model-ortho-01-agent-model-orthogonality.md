---
id: AGENT-MODEL-ORTHO-01
title: Agent–Model Orthogonality
type: principle
status: draft
implementation_state: specified_only
proposition: "Agent identity, state, mandate, tools, policy, and accountability should remain stable across dynamic changes in the model or cognitive mechanism selected for individual operations."
domains:
- agent
- models
- system
aliases:
- AGENT-MODEL-SEPARATION
relations:
- type: refines
  target: AGENT-DEF-01
- type: depends_on
  target: COG-GRAPH-01
- type: supports
  target: GONI-PRINCIPLE-HET-INTEL-01
sources: []
artifacts: []
uncertainty: "The optimal granularity of agent identity is application-specific. This principle establishes only that model identity is insufficient to define agent identity."
legacy: []
---

# Agent–Model Orthogonality

Goni should separate persistent agent identity from the cognitive substrate used
for each operation.

An agent is defined by persistent governed properties such as identity, state,
memory, mandate, policy, tools, budgets, and workflow.

For each operation, a cognitive mechanism may be selected dynamically according
to task, state, required function, evidence, and constraints.

A coding agent may therefore use deterministic Git inspection, a relevance
classifier, a bounded decision model, a small local generative model, a stronger
remote reasoner, and a verifier while remaining one accountable agent.

Changing the selected model or mechanism does not by itself change the agent's
identity, memory ownership, mandate, authority, or accountability.

Model switching must not reset or widen the current Work Order or capability
corridor.
