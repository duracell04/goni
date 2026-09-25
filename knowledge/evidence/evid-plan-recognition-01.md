---
id: EVID-PLAN-RECOGNITION-01
title: Plan recognition supports competing prospective task hypotheses
type: evidence
status: draft
implementation_state: not_applicable
proposition: Kautz and Allen's generalized plan-recognition framework supports representing observed behavior as evidence over possible plans rather than immediately collapsing observations into one asserted objective.
domains:
- research
- context
aliases: []
relations:
- type: supports
  target: ANTICIPATED-WORKORDER-01
sources:
- SRC-KAUTZ1986-PLAN-RECOGNITION
artifacts: []
uncertainty: The source predates modern LLM agents and does not prescribe Goni's WorkOrder representation; Goni adopts the competing-hypothesis principle.
legacy: []
---

# Plan recognition supports competing prospective task hypotheses

Generalized plan recognition treats observed actions as evidence about possible
plans. A central implication for Goni is epistemic discipline: observation may
support several plausible objectives at once.

An anticipated WorkOrder should therefore carry alternative objective
hypotheses when the distinction could change tools, risk, authority, or
consequential effects.
