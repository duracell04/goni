---
id: GONI-SYNTHESIS-83E91B709969
title: Virtual Cognitive Memory and Governed Delegation
type: synthesis
status: draft
implementation_state: specified_only
proposition: GONI should keep model-memory residency, cognitive-memory residency, and delegated authority as three independent hierarchies that interact through explicit mediation rather than collapsing into one control dimension.
domains:
- memory
- architecture
- authority
aliases: []
relations:
- type: synthesizes
  target: GONI-THESIS-AEA4F8746318
- type: synthesizes
  target: GONI-SPECIFICATION-C64D8275BA34
- type: synthesizes
  target: GONI-SPECIFICATION-BB656A72DF7D
- type: synthesizes
  target: GONI-THESIS-18044634B6FC
- type: refines
  target: GONI-SYNTHESIS-D5A359AF7D66
sources:
- SRC-PACKER2023-MEMGPT
- SRC-KWON2023-PAGEDATTENTION
- SRC-JUSTVUGG2026-COLIBRI
- SRC-GOOGLE2026-LITERT-LM
artifacts: []
uncertainty: This synthesis combines established external systems mechanisms with specified-only GONI abstractions. It does not claim that the complete architecture has been implemented or empirically validated.
legacy: []
---

# Virtual Cognitive Memory and Governed Delegation

The relevant architecture contains three orthogonal hierarchies.

## 1. Model-memory residency

Inference components may occupy faster or slower physical tiers such as accelerator memory, system RAM, and local storage. KV caches, model weights, experts, adapters, and multimodal encoders can be loaded, unloaded, shared, serialized, or prefetched. PagedAttention, Colibrì, and LiteRT-LM provide different precedents for treating data movement and residency as first-class systems concerns.

The objective of this hierarchy is computational efficiency. It says where inference state lives and how costly it is to access.

## 2. Cognitive-memory residency

GONI separately distinguishes active context from pinned, hot, warm-exact, and canonical recoverable state. This hierarchy concerns what evidence or state should be immediately available to reasoning and what may be recovered on demand.

The objective is task-relevant continuity under bounded context. The governing invariant is that demotion may change access cost while preserving provenance, epistemic status, and recoverability according to retention policy.

A semantic page fault occurs when required information is absent from active context but potentially recoverable. The specified response is retrieval, rehydration, and re-evaluation rather than unsupported reconstruction.

## 3. Authority

A third hierarchy governs what the delegated system may do: mandate, corridor, capability, mediated action, and receipt. This hierarchy is neither a memory cache nor a model-performance scale.

Retrieving more information, promoting a memory item, selecting a stronger model, or escalating to a remote model does not expand the current mandate. Authority changes require an explicit authorization path.

## Combined architecture

The three hierarchies can interact but must not collapse:

`physical/model residency ≠ cognitive residency ≠ authority`

A model may page in evidence without receiving tool access. A remote model may receive a redacted ContextPack without gaining access to the complete local archive. A locally pinned instruction may remain highly available while still being non-authoritative if it is merely a model-generated suggestion. Conversely, an authoritative constraint may remain in cold canonical state and still retain its normative status even when it is not currently resident in the model context.

The resulting compact doctrine is:

- **Active context is a cache, not memory.**
- **Summaries are indexes, not sources.**
- **Placement should determine latency, not truth.**
- **Model capability should determine reasoning quality, not authority.**
- **Models reason. The kernel authorizes. Tools act. Receipts prove.**

This framing converts long-context operation from a binary choice between retaining everything and compressing everything into a governed residency problem. It also preserves GONI's central sovereignty requirement: performance optimizations may alter where state is processed, but they do not silently relocate ownership of memory, policy, permissions, or evidence outside the GONI control boundary.
