---
id: GONI-IMAP-855D25F86358
title: Pro – Single big GPU exocortex
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Goal: maximum compatibility + throughput for CUDA-first stacks (vLLM, diffusion, etc.).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/25-hardware-layers-and-supplier-map.md
  heading: Pro – Single big GPU exocortex
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Pro – Single big GPU exocortex

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Pro – Single big GPU exocortex

Goal: maximum compatibility + throughput for CUDA-first stacks (vLLM, diffusion, etc.).

- Compute: 1× NVIDIA RTX-class GPU (24–32 GB VRAM), desktop CPU, 64–128 GB DDR5.
- Box: tower or larger SFF (acoustics become harder).
- Priority suppliers: **NVIDIA (default), EU integrators**.
