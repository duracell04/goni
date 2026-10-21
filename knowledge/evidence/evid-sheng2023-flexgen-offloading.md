---
id: EVID-SHENG2023-FLEXGEN
title: "FlexGen: I/O-aware inference offloading"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Sheng et al. (2023) demonstrate GPU-CPU-disk tensor placement, compression, and batch-oriented I/O scheduling for selected large-model inference workloads under constrained accelerator memory."
domains:
- hardware
- research
- software
aliases: []
relations:
- type: supports
  target: GONI-PRINCIPLE-9062425CD490
sources:
- SRC-SHENG2023-FLEXGEN
artifacts: []
uncertainty: "The reported gains target latency-insensitive, throughput-oriented workloads on evaluated hardware and models; they are not a general interactive-latency guarantee or a Goni implementation result."
legacy: []
---

# FlexGen: I/O-aware inference offloading

## Reported observation

Sheng et al. (ICML 2023) study high-throughput LLM inference with limited GPU
memory. FlexGen searches tensor placement and transfer schedules across GPU,
CPU, and disk, combined with compression and large effective batches. The
published evaluation reports results for selected hardware, models, and
throughput-oriented workloads.

## Goni interpretation and limits

Memory capacity, tensor placement, transfer bandwidth, batch size, and latency
objectives interact. A model that can execute under a small accelerator-memory
ceiling may be suitable for background batching but unsuitable for interactive
delegation. The scheduler should compare feasible execution profiles using
observed end-to-end costs on the actual target platform.
