---
id: GONI-PRINCIPLE-TRAJ-ECON-01
title: "Trajectory Economics"
type: principle
status: draft
implementation_state: specified_only
proposition: "Agent economics should be measured over complete trajectories because small per-step differences in context, inference, tools, verification, latency, privacy exposure, and energy compound across long-running delegated work."
domains:
- evaluation
- models
- performance
aliases: []
relations:
- type: refines
  target: GONI-SYNTHESIS-44FB7018181A
- type: supports
  target: GONI-PRINCIPLE-HET-INTEL-01
sources:
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "Exact cost terms and useful weighting depend on local hardware, provider pricing, caching, concurrency, verification burden, privacy policy, and task structure."
legacy: []
---

# Trajectory Economics

The economically relevant unit for delegated AI is successful work, not an
isolated model call.

For a trajectory with (N) steps, a minimal accounting model is:

[
C_{task}
=
\sum_{t=1}^{N}
\left(
C_{context,t}
+
C_{inference,t}
+
C_{tool,t}
+
C_{verification,t}
\right).
]

The system should additionally track dimensions that are not naturally reducible
to the same monetary unit:

- wall-clock latency and queueing delay;
- local energy or thermal cost where measurable;
- network and privacy exposure;
- external-dependency use;
- human interruption and review burden;
- rollback or compensation cost after incorrect action.

A locally cheaper inference can increase total task cost if it creates
additional iterations, verification work, tool failures, repeated context
reconstruction, or principal interruptions. Conversely, a more expensive
inference may improve whole-trajectory economics when it reliably shortens the
path to a verified result.

Accordingly, per-call cost and token efficiency remain useful operational
metrics, while promotion decisions should evaluate successful delegated work per
unit of money, time, energy, privacy exposure, and human attention where those
quantities can be measured.

No universal scalar weighting is assumed. Hard authority, privacy, and safety
constraints remain constraints rather than exchangeable economic terms.
