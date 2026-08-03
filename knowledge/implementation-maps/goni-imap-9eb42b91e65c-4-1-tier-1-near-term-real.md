---
id: GONI-IMAP-9EB42B91E65C
title: 4.1 Tier-1 (near-term, real)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**AMD (APU platform)** — critical for v1 APU-centric box.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/25-hardware-layers-and-supplier-map.md
  heading: 4.1 Tier-1 (near-term, real)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 4.1 Tier-1 (near-term, real)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Tier-1 (near-term, real)

- **AMD (APU platform)** — critical for v1 APU-centric box. Track ROCm/NPU tooling maturity.
- **NVIDIA (dGPU)** — default for Pro and many Max deployments due to CUDA ecosystem.
- **Intel (Gaudi / client NPUs)** — interesting for Max/alt vendor strategy; not an MVP dependency.
