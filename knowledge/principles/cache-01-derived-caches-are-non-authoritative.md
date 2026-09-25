---
id: CACHE-01
title: Derived Caches Are Non-Authoritative
type: principle
status: draft
implementation_state: specified_only
proposition: >-
  Performance caches and derived indexes may accelerate access to canonical GONI
  state but may never become the sole authoritative representation of memory,
  policy, provenance, permission, or task state.
domains:
- memory
- kernel
- inference
aliases:
- DERIVED-CACHE-TRUST-INVARIANT
relations:
- type: refines
  target: MEM-RETR-01
- type: refines
  target: CGG-01
- type: depends_on
  target: CTX-STATE-01
sources: []
artifacts: []
uncertainty: The invariant is architectural; exact reconstruction and invalidation mechanisms remain backend-specific.
legacy: []
---

# Derived Caches Are Non-Authoritative

> Status boundary: this is a new draft principle. It defines intended trust semantics, not an implemented cache invalidation mechanism.

GONI may use caches and derived indexes aggressively for performance. Examples include:

- KV and attention caches,
- prompt or prefix caches,
- ANN/vector indexes,
- sparse or lexical indexes,
- graph projections,
- retrieval caches,
- routing and scheduling caches.

These structures may be expensive to rebuild, but they are not canonical authority. Their loss or invalidation is permitted to reduce performance; it is not permitted to erase the underlying governed state.

The trust invariant is:

[
\text{cache loss} \Rightarrow \text{performance degradation}
]

and never:

[
\text{cache loss} \Rightarrow \text{semantic, policy, or authority loss}.
]

A cache entry MUST therefore be traceable to sufficient authoritative or governed inputs for reconstruction, revalidation, or safe omission. A cache MUST NOT become the only surviving copy of a user assertion, controlling rule, capability grant, receipt, provenance record, or resumable task state.

This generalizes the existing rule that graph databases, ANN indexes, and caches may be derived backends while canonical rows remain the local-first source of truth. The same discipline applies to inference-serving caches: acceleration may be disposable even when the underlying state is not.
