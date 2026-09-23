---
id: EVID-REACT-AGENT-LOOP-01
title: 'Source claim: ReAct interleaves reasoning and acting'
type: evidence
status: draft
implementation_state: not_applicable
proposition: ReAct demonstrates an agent-loop pattern in which language-model reasoning traces and task-specific actions are interleaved so that reasoning can update plans while actions gather information from external environments.
domains:
- research
- agents
aliases: []
relations:
- type: supports
  target: AGENT-ONTOLOGY-01
sources:
- SRC-YAO2023-REACT
artifacts: []
uncertainty: ReAct is a historical agent-loop reference and does not by itself define Goni's authority, memory, or runtime contracts.
legacy: []
---

# Source claim: ReAct interleaves reasoning and acting

Yao et al. study an interleaved reasoning-and-action pattern in which model-generated reasoning can track and revise plans while external actions return new information.

For Goni, this supports treating reasoning and acting as separable parts of an agent loop. Goni adds kernel-owned authority, persistent state, policy mediation, and receipts around that loop.
