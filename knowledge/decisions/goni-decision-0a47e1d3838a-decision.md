---
id: GONI-DECISION-0A47E1D3838A
title: Decision
type: decision
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: Hardware MVP remains APU-centric, **but** software must provide a validated inference backend for the APU reference hardware: either validate the ROCm path for the APU target, **or** implement a second backend (recommended) such as **llama.cpp** (Vulkan/HIP/CPU fallback).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Decision
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Decision

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Decision

- Hardware MVP remains APU-centric, **but** software must provide a validated inference backend for the APU reference hardware:
  - either validate the ROCm path for the APU target, **or**
  - implement a second backend (recommended) such as **llama.cpp** (Vulkan/HIP/CPU fallback).
