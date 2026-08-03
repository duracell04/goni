---
id: GONI-SPEC-C10CE9BB066F
title: 3.4 Continuous Cognition Envelope (latent-first OS loop)
type: specification
status: draft
implementation_state: specified_only
proposition: 'The hardware must support a low-power, always-on encoder loop plus bursty "solver" wakes: **Pinned shared memory** support across CPU/iGPU/NPU (or equivalent) so latent state can stay resident without page-fault spikes.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.4 Continuous Cognition Envelope (latent-first OS loop)
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.4 Continuous Cognition Envelope (latent-first OS loop)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Continuous Cognition Envelope (latent-first OS loop)

The hardware must support a low-power, always-on encoder loop plus bursty "solver" wakes:

- **Pinned shared memory** support across CPU/iGPU/NPU (or equivalent) so latent state can stay resident without page-fault spikes.
- **Coherence and synchronization** guarantees sufficient for a single-writer, multi-reader state update pattern.
- **Wake latency** for the big decoder path should be bounded and measurable (time to first action / first token).
- **Sensor ingest gating** should be feasible (default-off sensors, event-driven observation over constant polling where possible).
- **Write budget tolerance** for persistent state (SSD endurance and steady-state write patterns, not just peak throughput).
