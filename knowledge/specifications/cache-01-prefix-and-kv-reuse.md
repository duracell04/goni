---
id: CACHE-01
title: Prefix and KV Reuse
type: specification
status: draft
implementation_state: specified_only
proposition: Prefix and KV reuse is a runtime optimization over exactly compatible inference state; semantic similarity alone does not establish cache compatibility, and cached cognitive state never becomes authoritative policy state.
domains:
- software
- system
- kernel
aliases:
- prefix-cache-contract
relations:
- type: depends_on
  target: INF-FRAME-01
- type: depends_on
  target: GONI-DECISION-9AF49466170D
- type: refines
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-GIM2024-PROMPT-CACHE
- SRC-ZHENG2024-SGLANG
- SRC-KWON2023-VLLM
artifacts: []
uncertainty: Runtime engines expose different reuse semantics and KV layouts. Goni requires backend-specific conformance tests before treating a compatibility key as sufficient for safe reuse.
legacy: []
---

# Prefix and KV Reuse

Goni treats prompt-prefix reuse and KV-state reuse as execution/runtime
optimizations beneath semantic context compilation.

[
oxed{
	ext{semantic equivalence}

otRightarrow
	ext{cache compatibility}
}
]

## 1. Compatibility identity

A cache implementation SHOULD derive compatibility from an identity equivalent
to:

[
CacheKey =
H(
model,
adapter,
tokenizer,
template,
prefix_tokens,
position_semantics,
runtime_layout
).
]

The exact key format is backend-specific, but the compatibility decision MUST
account for every state component whose change can make reused attention state
invalid.

## 2. Invalidation

Reuse MUST be invalidated or revalidated when a material dependency changes,
including:

- model bundle;
- adapter or learned-prefix stack;
- tokenizer;
- chat template;
- exact reusable prefix token sequence;
- relevant position semantics;
- runtime KV representation or layout;
- stable system/policy prompt bundle when it participates in the prefix;
- tool-schema prefix when it participates in the reusable state.

A semantic paraphrase is a new token sequence unless the runtime proves an
equivalent reusable representation by another governed mechanism.

## 3. Four optimization axes

Goni measures four separate axes:

1. **prefix reuse** — avoid recomputing an identical compatible prefix;
2. **KV residency and paging** — manage where active cache blocks reside;
3. **sequence-state retention** — decide which prior states survive under
   constrained cache capacity;
4. **representation compression** — reduce bytes per surviving state.

An improvement on one axis MUST NOT be reported as evidence for another.

## 4. Observability

Runtime telemetry SHOULD distinguish:

- logical prompt tokens;
- fresh prefill tokens;
- reused prefix tokens;
- cache-hit and miss reasons;
- peak KV bytes;
- cache admission/eviction events;
- TTFT and decode latency;
- invalidation reason where a previously reusable prefix becomes ineligible.

## 5. Authority boundary

Cached cognition may accelerate classification, drafting, retrieval, or
planning. Before any consequential effect, authority MUST resolve against
canonical kernel-owned mandates, policy, capability, approval, budget, and
revocation state.

Cache reuse can reduce computation. It can never widen permission.
