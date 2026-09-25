---
id: GONI-EVIDENCE-FAST-SLOW-01
title: Adaptive routing between cheap and deliberative cognition has independent research support
type: evidence
status: draft
implementation_state: not_applicable
proposition: "System-1.x, Talker-Reasoner, and When to Reason independently support architectures that separate cheaper fast cognition from slower deliberative reasoning and activate the latter selectively, providing external grounding for Goni's ITCR and cognitive-graph escalation patterns."
domains:
- research
- models
- routing
aliases: []
relations:
- type: supports
  target: GONI-SPEC-FD6CC59ECC08
- type: supports
  target: COG-GRAPH-01
sources:
- SRC-SAHA2024-SYSTEM1X
- SRC-CHRISTAKOPOULOU2024-TALKER-REASONER
- SRC-WANG2025-WHEN-TO-REASON
artifacts: []
uncertainty: "The cited systems use different tasks, controllers, models, and definitions of fast/slow cognition. They support selective deliberation as a systems pattern, not one universal escalation threshold."
legacy: []
---

# Adaptive deliberation has independent research support

Several independent research lines separate routine/fast processing from more
expensive planning or reasoning.

System-1.x uses a controller to allocate planning sub-problems between faster
and slower modes. Talker-Reasoner separates a fast conversational role from a
slower planning/reasoning role. When to Reason uses semantic routing to invoke
reasoning mode only when predicted to be beneficial.

For Goni, the relevant conclusion is not literal psychological System 1/System
2 equivalence. It is the engineering pattern:

cheap cognition -> difficulty/uncertainty control -> expensive deliberation.

That pattern already exists in ITCR and can be represented generally by
COG-GRAPH-01.
