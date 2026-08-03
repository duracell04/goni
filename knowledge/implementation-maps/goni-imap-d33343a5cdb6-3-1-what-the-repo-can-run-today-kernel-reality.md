---
id: GONI-IMAP-D33343A5CDB6
title: 3.1 What the repo can run today (kernel reality)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: In goni-prototype-lab:software/kernel/goni-infer, the implemented engine is **HttpVllmEngine** (a client to a vLLM server).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/25-hardware-layers-and-supplier-map.md
  heading: 3.1 What the repo can run today (kernel reality)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 3.1 What the repo can run today (kernel reality)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 What the repo can run today (kernel reality)

In `goni-prototype-lab:software/kernel/goni-infer`, the implemented engine is **HttpVllmEngine** (a client to a vLLM server).
That means:

- **MVP end-to-end today is easiest on NVIDIA/CUDA**, because vLLM is most mature there.
- **APU-centric MVP requires either:**
  - validating vLLM on the target AMD APU/ROCm stack, or
  - adding a second runtime backend (recommended) such as **llama.cpp** (Vulkan/HIP/CPU fallback).

Actionable implication:
- Hardware MVP can still be APU-centric, but we must track the “runtime gap” explicitly and close it on the software side.
