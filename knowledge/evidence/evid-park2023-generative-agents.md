---
id: EVID-PARK2023-GENERATIVE-AGENTS
title: 'Source claim: Generative Agents'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Park et al. describe agents that record experience, synthesize memories into higher-level reflections, retrieve those memories for planning, and report ablation evidence that observation, reflection, and planning each contribute to believability in their simulation.
domains:
- research
- agent
- memory
aliases: []
relations:
- type: supports
  target: GONI-DECISION-6D3A2B71C9E4
- type: supports
  target: GONI-DECISION-FEB39440C565
sources:
- SRC-PARK2023-GENERATIVE-AGENTS
artifacts: []
uncertainty: The evaluation target is believable simulated behavior, not factual-memory integrity, delegated-action safety, or Goni's kernel governance.
legacy: []
---

# Source claim: Generative Agents

Park et al. combine an experience record, reflection over accumulated memories,
and planning in a simulated multi-agent environment.

## Goni relevance

This is direct precedent for periodic reflection over accumulated experience and
already informs Goni's Observation → Reflection → Planning lineage.

## Boundary

Believability in a simulation is not evidence of epistemic correctness or safe
delegated authority. DREAM-01 therefore adds boundaries the source does not
provide.
