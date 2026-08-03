---
id: GONI-IMAP-686FE25C071D
title: 1.1.3 dGPU topology (PCIe discrete GPU)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Benefits: Dedicated VRAM bandwidth and larger model capacity.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 1.1.3 dGPU topology (PCIe discrete GPU)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1.1.3 dGPU topology (PCIe discrete GPU)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1.3 dGPU topology (PCIe discrete GPU)

Benefits:
- Dedicated VRAM bandwidth and larger model capacity.

Risks:
- Copy overhead, wake latency, and idle power.

Invariants:
- Prohibit frequent host<->device state transfer.
- Only coarse-grained decision packets may cross PCIe.

Routing implications:
- Keep continuous cognition on CPU/NPU/iGPU; use dGPU only for bursts.

Telemetry needs:
- PCIe link state, GPU residency, and VRAM pressure signals.

Failure modes and fallbacks:
- PCIe transfer spikes -> reduce packet size and burst frequency.
- Thermal runaway -> clamp duty cycle and fall back to iGPU/CPU.

---
