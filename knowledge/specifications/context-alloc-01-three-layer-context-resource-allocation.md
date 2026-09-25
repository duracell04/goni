---
id: CONTEXT-ALLOC-01
title: Three-Layer Context Resource Allocation
type: specification
status: draft
implementation_state: specified_only
proposition: "Goni should evaluate source selection, semantic text compression, and model-internal KV residency as three distinct context-allocation layers with separate objectives, failure modes, evidence, and recovery mechanisms."
domains:
- memory
- models
- software
- specs
aliases:
- THREE-LAYER-CONTEXT-ALLOCATION
relations:
- type: refines
  target: GONI-PRINCIPLE-9062425CD490
- type: refines
  target: GONI-IMAP-66D2DBD94204
- type: refines
  target: GONI-SYNTHESIS-227700D474C6
sources: []
artifacts: []
uncertainty: "The boundaries are conceptual and may be fused by a concrete runtime. Evaluation should still attribute gains and losses to the layer where information is discarded or compressed."
legacy: []
---

# Three-Layer Context Resource Allocation

Goni should distinguish three different optimization surfaces that are often
collapsed under the term context compression.

## L1 — Source and evidence selection

Question:

Which documents, chunks, memories, facts, or observations should enter the
candidate context at all?

Examples include retrieval, graph expansion, reranking, policy filtering, and
submodular context selection.

The principal failure mode is omission of decisive evidence.

Recovery commonly requires broader retrieval or another source-selection pass.

## L2 — Semantic text compression

Question:

Given selected source material, which textual spans, tokens, statements, or
structured facts should be retained, compressed, or omitted before model
prefill?

This layer may use extractive retention, query-aware compression, structured
fact projection, or bounded summarization.

The principal failure mode is semantic information loss or distortion inside
material that was correctly selected at L1.

Compression artifacts SHOULD preserve source refs and enough provenance to
rehydrate original evidence when verification or later reasoning requires it.

## L3 — Model-internal residency

Question:

After text has entered the model, which KV states or other internal
representations remain resident, at what fidelity, and for how long?

Examples include paging, token/state eviction, heavy-hitter retention,
architecture-specific latent compression, and precision/fidelity allocation.

The principal failure mode is loss of model-accessible long-range dependencies
despite the original text having been supplied.

Recovery depends on runtime capabilities and may require re-prefill or
rehydration rather than another semantic retrieval pass.

## Separation rule

The layers are related but non-equivalent:

source selection != semantic text compression != KV residency.

An improvement at one layer does not establish an improvement at another.

For example:

- a smaller KV cache does not prove that source selection improved;
- a compressed prompt does not prove that removed evidence was irrelevant;
- broader retrieval does not require retaining every candidate token in model
  context.

## Evaluation

Each layer SHOULD report layer-appropriate metrics.

L1 may measure candidate recall, citation coverage, retrieval latency, and
context-selection quality.

L2 may measure retained token ratio, answer quality, factual preservation,
source-span recoverability, and compression latency.

L3 may measure peak KV memory, bytes moved, TTFT/decode latency, throughput,
long-range recall, and re-prefill cost.

Cross-layer experiments SHOULD keep the other layers fixed where practical so
the source of gains and degradation remains identifiable.

## Authority boundary

All three layers are cognitive optimizations.

Lossy selection, compression, or KV retention MUST NOT become the sole source
for mandates, capabilities, revocation, approvals, or other canonical authority
state.
