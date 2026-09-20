---
id: GONI-PRINCIPLE-SYS-CAP-01
title: "System Capability Is System-Level"
type: principle
status: draft
implementation_state: specified_only
proposition: "Observed AI capability is a property of the complete execution system—model, harness, canonical state, tools, environment, and verification—not of the model alone."
domains:
- agent
- evaluation
- system
aliases: []
relations:
- type: refines
  target: DOCTRINE-DELEG-01
- type: supports
  target: GONI-THESIS-7BFB74017D50
sources:
- SRC-LIN2026-AGENTIC-HARNESS-ENGINEERING
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "Specified design principle. The exact contribution of each system component is workload- and model-dependent and must be measured under controlled evaluation."
legacy: []
---

# System Capability Is System-Level

Goni evaluates an AI system at the level of the complete execution loop rather than attributing observed behavior to the model alone.

A useful abstraction is:

[
C_{system} = f(M,H,S,T,E,V)
]

where (M) is the model, (H) the harness, (S) canonical state and context management, (T) tools and action interfaces, (E) the execution environment, and (V) verification.

This principle requires two distinct comparison modes. **Model comparisons** hold the surrounding scaffold approximately constant. **System comparisons** evaluate each complete configuration as deployed. Results from one mode must not be presented as if they answered the other.
