---
id: CTX-LIFE-01
title: Bounded Context Lifecycle
type: specification
status: draft
implementation_state: specified_only
proposition: >-
  Active model context is a bounded execution resource with an explicit
  lifecycle; GONI should roll contexts over through governed checkpointing,
  eviction, and reconstruction rather than accumulate transcripts indefinitely.
domains:
- context
- software
- scheduler
aliases:
- CONTEXT-ROLLOVER
relations:
- type: refines
  target: GONI-SPEC-33294E0D3306
- type: depends_on
  target: GONI-SPEC-B49DB23CF412
- type: depends_on
  target: CTX-STATE-01
- type: depends_on
  target: CACHE-01
sources: []
artifacts: []
uncertainty: Rollover thresholds and policies require matched evaluation across model and runtime families.
legacy: []
---

# Bounded Context Lifecycle

> Status boundary: this is a new specified-only contract. Thresholds, scheduler integration, and reconstruction behavior remain to be implemented and tested.

Logical interaction continuity does not require physical model-context continuity. A user may experience one continuous task or conversation while GONI uses multiple bounded inference episodes underneath.

The intended lifecycle is:

[
\text{OPEN}
\rightarrow
\text{ACTIVE}
\rightarrow
\text{PRESSURE}
\rightarrow
\text{CHECKPOINT}
\rightarrow
\text{EVICT}
\rightarrow
\text{REHYDRATE}.
]

A context enters `PRESSURE` according to resource and information conditions rather than a single hard-coded token count. Relevant signals MAY include:

- token occupancy and reserved generation headroom,
- KV-cache bytes and fragmentation,
- prefill latency,
- available RAM, VRAM, or unified memory,
- scheduler queue pressure and concurrent resident workloads,
- evidence density and context redundancy,
- retrieval confidence and unresolved source requirements,
- model-specific long-context degradation or positional sensitivity.

When rollover is selected, the Control Plane SHOULD create a resumable checkpoint, preserve source and receipt references, release disposable inference state, and later compile a fresh `ContextPack` from the checkpoint plus current governed state.

Rehydration MUST NOT blindly replay a stale prompt. It should combine:

1. the bounded task checkpoint,
2. current policy and authority state,
3. current durable memory,
4. fresh retrieval appropriate to the Work Order, and
5. only the recent raw interaction required for local conversational coherence.

Maximum supported context length is therefore a capability ceiling, not a target operating point. GONI may intentionally use a shorter active context when doing so improves task quality, latency, concurrency, energy use, or reconstructibility.
