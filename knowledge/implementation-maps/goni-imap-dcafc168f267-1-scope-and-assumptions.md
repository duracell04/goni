---
id: GONI-IMAP-DCAFC168F267
title: 1. Scope and assumptions
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'This BOM assumes: Single-board compute module with CPU + iGPU + NPU and 128 GB unified LPDDR5X Quiet small-form-factor PSU for sustained APU workloads Two NVMe SSDs (OS/containers and data/models) Mainstream cooling path (air or 240 mm AIO) Custom enclosure and front-panel MCU'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/50-bom-experiments/bom-v1-apu-node.md
  heading: 1. Scope and assumptions
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1. Scope and assumptions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope and assumptions

This BOM assumes:
- Single-board compute module with CPU + iGPU + NPU and 128 GB unified LPDDR5X
- Quiet small-form-factor PSU for sustained APU workloads
- Two NVMe SSDs (OS/containers and data/models)
- Mainstream cooling path (air or 240 mm AIO)
- Custom enclosure and front-panel MCU
