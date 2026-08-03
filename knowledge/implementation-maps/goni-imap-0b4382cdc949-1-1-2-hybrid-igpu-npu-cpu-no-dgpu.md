---
id: GONI-IMAP-0B4382CDC949
title: 1.1.2 Hybrid iGPU + NPU + CPU (no dGPU)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Benefits: Low idle power and predictable always-on behavior.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 1.1.2 Hybrid iGPU + NPU + CPU (no dGPU)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1.1.2 Hybrid iGPU + NPU + CPU (no dGPU)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1.2 Hybrid iGPU + NPU + CPU (no dGPU)

Benefits:
- Low idle power and predictable always-on behavior.

Risks:
- Limited peak reasoning throughput for large bursts.

Invariants:
- NPU is used only for fixed-graph, shape-bucketed workloads.
- iGPU is reserved for burst reasoning when memory-bound.

Routing implications:
- Encoders map to NPU buckets; fallback to CPU/iGPU on shape mismatch.

Telemetry needs:
- NPU supported shapes and graph cache status.
- GPU wake/active state.

Failure modes and fallbacks:
- NPU graph mismatch -> route to CPU/iGPU.
- Burst overruns -> extend cooldown and reduce duty cycle.
