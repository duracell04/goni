---
id: GONI-IMAP-14C0E0DB2663
title: 2.1 Responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'For the MVP, the substrate is responsible for: **Process model** Start goni-http (or goni-node) on boot (systemd, Docker, k8s, …).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/os-and-base-image.md
  heading: 2.1 Responsibilities
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 2.1 Responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Responsibilities

For the MVP, the substrate is responsible for:

- **Process model**
  - Start goni-http (or goni-node) on boot (systemd, Docker, k8s, …).
  - Capture logs via stdout/stderr (journald or container logs).

- **Resource exposure**
  - Make CPU cores available to Rust/Arrow/Wasm.
  - Optionally expose GPU/NPU devices to the LLM runtime (via CUDA/ROCm/Metal/NPUs).


- **Memory and device hygiene**
  - Provide a way to pin/lock latency-critical pages (latent state, encoder buffers).
  - Allow swap to be disabled or encrypted for state pages.
  - Enable IOMMU or equivalent DMA protection when available.

- **Storage**
  - Provide *persistent* directories for:
    - model weights (e.g. /opt/goni/models),
    - data plane (e.g. /var/lib/goni for indices, metrics),
    - configuration (e.g. /etc/goni or $XDG_CONFIG_HOME/goni).
