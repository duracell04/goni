---
id: GONI-IMAP-335837AD055B
title: 2.1 Responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Inference execution** Convert a PromptPlan + ModelId into a token stream.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 2.1 Responsibilities
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 2.1 Responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Responsibilities

- **Inference execution**
  - Convert a PromptPlan + ModelId into a token stream.

- **Bundle loading and attestation**
  - Load only declared bundle IDs produced by the promotion pipeline.
  - Surface the active trunk version, expert mesh version, and patch set hashes
    for audit and receipts.

- **Model capability description**
  - Report per-model:
    - max_context,
    - nominal tokens/s,
    - memory footprint / device requirements,
    - supported shape buckets and graph-compile constraints (for NPUs),
    - KV-cache limits and paging mode (contiguous vs segmented),
    - optional speculative/draft-model support: compatible draft bundles,
      maximum draft length, confidence-head availability, and verifier batch
      constraints.

- **Utilisation reporting**
  - Track and expose current load (per model and per device) to scheduler / resman in ??.
  - Expose effective bandwidth and memory pressure signals for routing decisions.

- **Cancellation / preemption hooks**
  - Support cooperative cancellation so ?? can abort or delay jobs.

- **Wake and warm-state control**
  - Report cold-start latency and warm state per model/device.
  - Support pre-warm and keep-alive budgets so decoder wake is bounded.
  - Support shape-bucket pre-warm for NPU graph stability.

### Memory-hierarchy placement profiles (INF-PLACE-01)

Each runtime adapter SHOULD expose its supported resident, offloaded,
layer-streamed, and expert-streamed modes, including supported architecture,
bundle/precision combinations and hardware dependencies. Per-profile estimates
SHOULD distinguish weight residency, KV/activation memory, free storage needed,
model preparation, cold/warm startup, transfer traffic, latency, and energy,
using a named hardware/workload measurement where available.

The adapter reports measured memory pressure, wall time, storage reads/writes,
and execution provenance to the kernel's existing metering and receipt path.
The kernel retains admission, budget enforcement, scheduling, and cancellation
authority. Internal prefetching and tensor-level transfers remain backend-owned
operations, subject to kernel-visible resource use and backpressure.
