---
id: GONI-SPEC-ED247336E836
title: 3.5 Continuous cognition envelope (hardware-linked)
type: specification
status: draft
implementation_state: specified_only
proposition: Continuous cognition (encoders + predictor) must fit a steady-state power and thermal budget; heavy solvers are interrupt-only.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.5 Continuous cognition envelope (hardware-linked)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.5 Continuous cognition envelope (hardware-linked)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 Continuous cognition envelope (hardware-linked)

- Continuous cognition (encoders + predictor) must fit a steady-state power and
  thermal budget; heavy solvers are interrupt-only.
- Thermal scheduling should clamp compute bursts near the efficiency frontier
  of DVFS curves, prioritizing sustained performance over short peaks.
- The scheduler must enforce wake hysteresis and a maximum solver wake rate
  per policy.
- SSD endurance is a first-class constraint; write amplification must be
  mitigated via RAM-first deltas, significance thresholds, and deferred
  compaction.
- Prefer UMA/shared-memory paths; avoid PCIe shuttling of latent state. If a
  discrete GPU is used, fallback rules must keep continuous cognition off dGPU
  by default.
- Control-plane arbitration should use zero-copy, schema-driven interchange
  (Arrow/FlatBuffers/Cap'n Proto); avoid JSON in hot paths between CPU and accelerators.
- Sensors must be gated and default-off; each source requires explicit policy
  enablement.
- Crash consistency is mandatory: state must be replayable from checkpoints +
  append-only logs; journaling is required for durable records.
