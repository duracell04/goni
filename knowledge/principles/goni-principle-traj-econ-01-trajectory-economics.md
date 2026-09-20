---
id: GONI-PRINCIPLE-TRAJ-ECON-01
title: "Trajectory Economics"
type: principle
status: draft
implementation_state: specified_only
proposition: "Agent economics should be measured over complete trajectories because small per-step differences in context, inference, tools, latency, and energy compound across long-running delegated work."
domains:
- evaluation
- models
- performance
aliases: []
relations:
- type: refines
  target: GONI-SYNTHESIS-44FB7018181A
- type: depends_on
  target: GONI-PRINCIPLE-TRAJ-EVAL-01
sources:
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "Exact cost terms depend on local hardware, provider pricing, caching behavior, concurrency, and task structure."
legacy: []
---

# Trajectory Economics

For an agent trajectory with (N) steps:

[
C_{task} approx sum_{t=1}^{N}(C_{input,t}+C_{reasoning,t}+C_{output,t}+C_{tool,t})
]

Goni should additionally track latency and, for local execution where measurable, energy or thermal cost.

Useful system metrics include successful delegated work per unit of money, wall-clock time, and energy.
