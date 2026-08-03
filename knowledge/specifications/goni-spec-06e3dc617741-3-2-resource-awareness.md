---
id: GONI-SPEC-06E3DC617741
title: 3.2 Resource Awareness
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system must: Monitor its own CPU, memory, storage, and accelerator utilisation.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.2 Resource Awareness
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.2 Resource Awareness

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Resource Awareness

- The system must:
  - Monitor its own CPU, memory, storage, and accelerator utilisation.
  - Avoid overloading the device to the point of becoming unresponsive.
  - Degrade gracefully under high load (e.g. queue tasks, slow down background jobs).
  - Provide a **deterministic inference mode** for audit/self-loop workloads:
    - temperature = 0, fixed seed, batch size 1, no continuous batching.
    - single worker / single thread (or CPU-only fallback) available even if slower.
    - record blueprint/hardware/driver profile with runs so outputs can be reproduced.
  - Treat transformer decoding and tool loops as **memory-bandwidth bound** by
    default; scheduling should prioritize bandwidth per effective token, not
    peak TOPS/FLOPS.
  - Optimize **KV-cache residency and access patterns** (paged/segmented layouts)
    as a first-class performance constraint.
  - Route memory-bound stages to the highest-bandwidth path available (GPU/iGPU/UMA CPU)
    and avoid avoidable PCIe shuttling of KV and latent state.
  - Respect accelerator **shape constraints**: NPUs are used only when workloads
    fit fixed-shape or bucketed regimes; otherwise route to CPU/iGPU.
