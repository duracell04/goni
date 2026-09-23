---
id: AGENTICNESS-01
title: Agenticness as delegated sequential authority
type: glossary
status: draft
implementation_state: specified_only
proposition: In Goni, agenticness means the degree to which a system is delegated authority to make and execute sequential decisions over time within a mandate, rather than the number of agents or the presence of a particular model technique.
domains:
- product
- system
- agents
aliases:
- agentic AI
- agenticness
relations:
- type: refines
  target: GONI-THESIS-826C859B5D30
sources:
- SRC-PLAAT2025-AGENTIC-LLM-SURVEY
- SRC-ZHANG2026-AGENTIC-RL
artifacts: []
uncertainty: The term agentic AI remains inconsistently defined across academia and industry; this is Goni's operational definition.
legacy: []
---

# Agenticness as delegated sequential authority

Goni uses **agenticness** as a continuous system property.

Higher agenticness means that, within an explicit mandate and authority corridor, the runtime may independently perform more of the following over a longer horizon:

- acquire observations,
- update task and belief state,
- choose among actions,
- invoke permitted tools,
- verify outcomes,
- recover or replan,
- decide when to continue,
- decide when uncertainty requires human escalation.

This definition does not imply that more autonomy is always preferable. Goni's design objective is the greatest useful delegated autonomy consistent with policy, evidence, reversibility, user attention, and consequence.

A single-agent runtime can therefore be highly agentic, while a multi-agent system can be minimally agentic if every step requires human approval.
