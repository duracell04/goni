---
id: WORLD-MODEL-01
title: Consequence simulation as an optional planning layer
type: proposal
status: draft
implementation_state: specified_only
proposition: Goni should study optional world-model or consequence-simulation components for expensive or irreversible actions so candidate actions can be evaluated against predicted state transitions before execution without making simulation an authority source.
domains:
- research
- planning
- agents
aliases:
- world model
- consequence simulation
relations:
- type: refines
  target: AGENT-STATE-01
sources:
- SRC-YANG2026-WORLD-MODELS
artifacts: []
uncertainty: This is a research proposal. Predicted consequences can be wrong, so simulation must never substitute for kernel policy, actual execution receipts, or postcondition verification.
legacy: []
---

# Consequence simulation as an optional planning layer

For high-cost actions, the planner may benefit from evaluating candidate consequences before committing an effect.

A research shape is:

```text
current belief/task state
  -> candidate actions
  -> consequence/world model
  -> predicted state transitions and uncertainty
  -> planner comparison
  -> kernel authorization
  -> real execution
  -> actual observation and verification
```

Potential target domains include:

- destructive filesystem or database operations,
- expensive purchases or commitments,
- migrations and deployments,
- privacy-sensitive disclosure,
- robotics and embodied actions,
- long-running scientific or compute jobs.

## Authority boundary

The world model is advisory cognition. It receives no independent authority.

A predicted safe outcome cannot authorize an action. The kernel still evaluates policy and capabilities, and the post-action monitor verifies the real environment because model predictions may be wrong.

## Promotion criterion

This layer should be promoted only if controlled Goni Lab experiments show that it reduces consequential errors or interaction cost enough to justify added latency, complexity, and model error risk.
