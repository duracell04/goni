---
id: EVID-AGENTIC-RL-POMDP-01
title: 'Source claim: agentic RL is modeled as temporally extended partial observability'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Zhang et al. distinguish single-step LLM reinforcement learning from temporally extended agentic interaction and formalize the latter using partially observable Markov decision processes.
domains:
- research
- agents
- state
aliases: []
relations:
- type: supports
  target: AGENT-STATE-01
sources:
- SRC-ZHANG2026-AGENTIC-RL
artifacts: []
uncertainty: Goni adopts this as a conceptual vocabulary for state, observation, belief, action, and uncertainty; it does not imply that the current runtime implements a POMDP solver.
legacy: []
---

# Source claim: agentic RL is modeled as temporally extended partial observability

The Agentic RL survey frames agent behavior as sequential interaction in partially observable environments.

Goni can use that formalism to distinguish world state, observations, internal state or beliefs, actions, and objectives without committing the implementation to a particular reinforcement-learning algorithm.
