---
id: GONI-SYNTHESIS-585026C9411D
title: 3.1 Mediation at tool syscall boundary
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Every effectful tool operation must pass the same mediation choke point: capability check, policy and budget decision, egress classification (if applicable), receipt emission.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 3.1 Mediation at tool syscall boundary
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.1 Mediation at tool syscall boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Mediation at tool syscall boundary

Every effectful tool operation must pass the same mediation choke point:
- capability check,
- policy and budget decision,
- egress classification (if applicable),
- receipt emission.

Related foundations:
- reference monitor formulation [[anderson1972-reference-monitor]]
- protection model framing [[lampson1974-protection]].
