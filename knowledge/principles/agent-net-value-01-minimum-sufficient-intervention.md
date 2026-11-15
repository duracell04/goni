---
id: AGENT-NET-VALUE-01
title: 'Agentic Net Value and Minimum Sufficient Intervention'
type: principle
status: draft
implementation_state: specified_only
proposition: 'Agentic intelligence should be evaluated by the net value of the outcomes it produces, including the costs and risks of its interventions. Greater capability should improve the system''s ability to identify and perform the smallest sufficient action, while recognizing when further work has negative expected marginal value.'
domains:
- agent
- kernel
- system
aliases: []
relations:
- type: refines
  target: GONI-SPEC-59E3DFC23CC8
- type: supports
  target: GONI-PRINCIPLE-9062425CD490
sources: []
artifacts: []
uncertainty: 'Marginal benefits and opportunity costs are uncertain and task-dependent; this is a design principle rather than a validated numerical decision rule.'
legacy: []
---

# Agentic Net Value and Minimum Sufficient Intervention

> Agentic intelligence should be evaluated by the net value of the outcomes it produces, including the costs and risks of its interventions. Greater capability should improve the system's ability to identify and perform the smallest sufficient action, while recognizing when further work has negative expected marginal value.

## Operational interpretation

An agent-generated subtask belongs to an active Work Order if it advances an unmet DoneContract criterion, resolves material uncertainty, or protects a binding safety or quality requirement. Optional interventions should proceed when their expected incremental value justifies their full costs: compute, human oversight, verification, retries, integration, maintenance, and opportunity costs.

Prefer the least complex sufficient authorized intervention. Once required outcomes and verification conditions are satisfied, terminate the active task. Present further improvements as proposals for an explicitly authorized scope, unless the current mandate already authorizes continued optimization within defined budgets.

Marginal-value estimates are uncertain and need not require a separate scoring engine. Safety, legal authority, and mandatory verification are binding constraints rather than discretionary improvements.

This principle extends the existing DoneContract and smallest-sufficient cognitive resource allocation without changing capability, tool, or egress authority.
