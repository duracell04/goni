---
id: CTX-STATE-01
title: Memory, Context, and KV State Are Distinct
type: principle
status: draft
implementation_state: specified_only
proposition: >-
  GONI must distinguish durable governed memory, task-specific context, model
  token context, and runtime KV/attention state; KV state is reconstructible
  computational cache rather than canonical personal memory or authority-bearing
  state.
domains:
- memory
- software
- inference
aliases:
- COMPUTATIONAL-MEMORY-HIERARCHY
relations:
- type: refines
  target: GONI-PRINCIPLE-B40DDEFD1872
- type: depends_on
  target: MEM-RETR-01
sources: []
artifacts: []
uncertainty: This is a specified architectural distinction; backend-specific cache semantics require runtime validation.
legacy: []
---

# Memory, Context, and KV State Are Distinct

> Status boundary: this is a new draft principle. It specifies an architectural distinction; it does not claim that any current GONI runtime enforces it.

GONI treats four forms of state as different system objects:

| Layer | Typical lifetime | Canonical authority | Reconstruction basis |
| --- | --- | --- | --- |
| Durable governed memory | Long-lived | Yes, subject to memory policy | Canonical Memory/Knowledge Plane |
| `ContextPack` | One Work Order or task episode | No; derived and receipt-linked | Governed retrieval, policy, and compression |
| Model token context | One inference episode | No | Materialized `ContextPack`, current instructions, and task state |
| KV / attention state | Runtime-dependent | No | Recompute from the model input and model/runtime state |

The distinction is operational, not merely terminological. A KV cache records computation already performed over an active token sequence. It can reduce prefill or decode cost, but it does not become personal memory merely because it persists for some period. Likewise, a long model context is not durable memory simply because the model can attend to it.

The core invariant is:

[
\text{loss of derived inference state}
\Rightarrow
\text{recomputation cost, not canonical information loss}.
]

Deleting or evicting KV state may increase latency. It MUST NOT delete or silently alter durable memory, policy, authority, provenance, receipts, or the reconstructible state required to resume a Work Order.

This principle extends the existing proposition that context is scarce working memory rather than durable storage. GONI therefore keeps identity and continuity outside the model runtime and permits model backends to be replaced without transferring ownership of memory to the model.
