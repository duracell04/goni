---
id: GONI-DECISION-9AF49466170D
title: Probabilistic cognition, deterministic authority
type: decision
status: draft
implementation_state: specified_only
proposition: Lossy, approximate, probabilistic, compressed, cached, or evictable representations may assist cognition but may not constitute the authoritative source of mandates, capabilities, policy, revocation, or other kernel-owned authority state.
domains:
- agent
- kernel
- security
- system
aliases:
- authority-state-is-not-a-cache
relations:
- type: refines
  target: GONI-THESIS-7BFB74017D50
- type: depends_on
  target: GONI-PRINCIPLE-9062425CD490
sources: []
artifacts: []
uncertainty: This decision specifies an architectural boundary. Concrete canonical stores, signatures, replication, and recovery semantics remain governed by their respective authority and receipt specifications.
legacy: []
---

# Probabilistic cognition, deterministic authority

Goni may aggressively optimize the representations used for reasoning:

- approximate retrieval;
- embeddings and ANN indexes;
- summaries and distilled state;
- compressed or quantized memory representations;
- KV-cache paging, retention, or eviction;
- prefix caches and exact compatible prefix/KV reuse;
- learned relevance or reuse estimates;
- sparse model or tool routing.

Those mechanisms are cognitive accelerators. They are not the source of
authority.

The authority path MUST resolve against canonical kernel-owned state for
mandates, policy, capabilities, approval corridors, budgets, revocation, and
other facts that determine whether an effect is permitted.

The governing rule is:

[
oxed{
	ext{probabilistic allocation for cognition}
+
	ext{canonical authority for effects}
}
]

A cached or summarized authority-related representation MAY help the system
prepare a proposal, classify a task, or prioritize work. Before consequential
execution, the kernel MUST evaluate the authoritative state required by the
relevant policy contract.

The same rule applies to `ContextPack`, `InferenceFrame`, prefix caches, and
KV state. These representations may carry or accelerate information about
authority, but they do not become the authoritative source merely because they
are model-visible or computationally resident.

This boundary prevents an efficiency mechanism from silently changing
permissions. Cache eviction, approximation error, stale summaries, compression
loss, or cache reuse may reduce answer quality or alter cognition; they MUST NOT
widen authority.

Any future optimization that makes policy, mandate, capability, revocation, or
approval decisions depend exclusively on lossy or cached cognitive state
requires an explicit replacement decision and a new safety argument.
