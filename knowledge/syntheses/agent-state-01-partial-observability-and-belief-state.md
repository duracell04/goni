---
id: AGENT-STATE-01
title: Partial observability and agent state
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni should distinguish environment state, observations, task state, belief state, memory, and model context because a delegated agent acts under partial and potentially stale information.
domains:
- system
- research
- agents
- state
aliases:
- belief state
- partial observability
relations:
- type: refines
  target: GONI-THESIS-E1FB8B4F7772
sources:
- SRC-ZHANG2026-AGENTIC-RL
- SRC-HU2026-AGENT-MEMORY-SURVEY
artifacts: []
uncertainty: This node adopts POMDP vocabulary as a conceptual model for uncertainty and state separation; it does not specify or imply a POMDP solver in the implementation.
legacy: []
---

# Partial observability and agent state

A delegated agent rarely sees the full environment state. Goni therefore treats the following concepts as distinct:

- **Environment state (s_t):** the real external condition at time (t), including facts the agent may not be able to observe directly.
- **Observation (o_t):** information exposed to the agent through an interface, tool result, sensor, API response, CLI output, screenshot, event, or other observation channel.
- **Belief state (b_t):** the agent's current uncertainty-bearing representation of what is probably true about the environment, derived from prior observations, memory, and inference.
- **Task state:** explicit workflow progress such as completed steps, pending obligations, checkpoints, retries, budgets, and open preconditions.
- **Memory:** persistent information available across tasks or sessions under Goni's memory-governance rules.
- **Model context:** the bounded information payload supplied to a particular model inference.

These objects may overlap in content while remaining different in authority and lifecycle.

A useful conceptual form is:

[
o_t \sim O(s_t), \qquad
b_{t+1} = U(b_t, o_{t+1}, a_t), \qquad
a_t \sim \pi(a_t \mid b_t, g)
]

where (g) is the current objective or mandate. This notation is explanatory, not an implementation commitment.

## Consequences for Goni

1. A tool result is an observation, not automatically ground truth.
2. Memory may be stale, contradicted, superseded, or lower-authority than the current environment.
3. When consequential state is uncertain, information gathering can be the correct next action.
4. Verification should inspect the post-action environment rather than assuming that a requested action succeeded.
5. Clarification or human escalation can be a competent action when the information required for safe delegation is unavailable.
6. Perception adapters should normalize observation provenance and confidence before observations influence durable memory or consequential action.

This framing extends Goni's existing latent-state and authority architecture with an explicit epistemic model for what the system knows, what it observes, and what remains uncertain.
