---
id: EVID-AIRLLM-STREAMING
title: "AirLLM: layer and expert streaming on constrained accelerators"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "AirLLM project documentation reports layer-wise dense-model and per-expert sparse-MoE streaming that reduce peak accelerator residency while increasing dependence on storage and data transfer."
domains:
- hardware
- research
- software
aliases: []
relations:
- type: supports
  target: GONI-PRINCIPLE-9062425CD490
sources:
- SRC-AIRLLM-REPO
artifacts: []
uncertainty: "The source documents functionality and project-reported minimum-VRAM demonstrations; comparative latency, energy, storage traffic, model accuracy, and Goni hardware compatibility are not independently established."
legacy: []
---

# AirLLM: layer and expert streaming on constrained accelerators

## Reported observation

AirLLM describes loading model layers sequentially, executing each on an
accelerator, and releasing their working weights. Its sparse-MoE path reports
loading routed experts rather than keeping all experts resident. The project
also describes prefetching and optional 4-bit or 8-bit compression.

These mechanisms distinguish the bytes needed for one execution step from
the total model-parameter footprint. Successful initialization or generation
within a small peak VRAM budget is evidence of execution feasibility, not
necessarily useful interactive speed.

## Goni interpretation and limits

This supports testing memory-residency strategies independently from model
selection and numerical precision. Dense models can repeatedly stream large
weight volumes during decoding; sparse expert routing has a different transfer
profile. Model preparation, available disk capacity, CPU memory, storage
bandwidth, context/KV cache, and energy remain material constraints.

Promotion to a Goni runtime profile requires measured results on an identified
model, software version, workload, and hardware configuration.
