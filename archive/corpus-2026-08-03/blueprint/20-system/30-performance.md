# Performance Realism for Sequence Architectures
DOC-ID: SYS-02
Status: Specified only / roadmap

This document defines how Goni evaluates claims about alternative sequence
architectures (sub-quadratic attention, state-space, recurrent hybrids).

Goal: keep architecture decisions grounded in realistic tokenizer regimes,
scale, wall-clock performance, and product-level constraints.

## 1. Toy benchmark results are not replacement evidence

Small-model wins on toy datasets are useful for ablation, not sufficient for
claims of practical transformer replacement.

Evaluation implication:
- treat tiny char-level WikiText-2 results as proof-of-concept only,
- require scale-up evidence on realistic corpora and model sizes before
  architecture-level conclusions.

Related work:
- S4 long-sequence evaluation framing [[gu2021-s4]]
- RWKV scaling evidence to larger model sizes [[peng2023-rwkv]]
- Hyena hierarchy large-scale comparisons [[poli2023-hyena]].

## 2. Tokenization realism is a first-class criterion

Tokenizer regime changes architecture behavior:
- character-level setups inflate sequence length and alter optimization
  dynamics,
- modern LLM quality baselines are built on subword tokenization.

Evaluation implication:
- report quality under realistic tokenization (BPE/SentencePiece-like),
- treat large regressions under subword settings as non-competitive until
  resolved.

Related work:
- RWKV large-scale LM evaluations [[peng2023-rwkv]]
- Hyena LM evaluations with mainstream corpora [[poli2023-hyena]].

## 3. Big-O claims must be validated against optimized baselines

Asymptotic complexity alone is insufficient. IO-aware kernels can dominate in
practical ranges despite less favorable big-O.

Evaluation implication:
- compare against strong optimized attention baselines,
- report prefill/decode throughput, latency, and memory at multiple sequence
  lengths,
- identify explicit crossover points where alternative operators become faster.

Related work:
- FlashAttention IO-aware speed and memory behavior [[dao2022-flashattention]]
- Hyena operator crossover behavior and long-context scaling [[poli2023-hyena]].

## 4. Novelty bar: this direction is established

Replacing or reducing explicit attention has extensive prior art:
- structured state-space approaches,
- implicit long convolutions,
- recurrent-transformer hybrids,
- retentive alternatives.

Evaluation implication:
- "sub-quadratic" is not itself a contribution,
- contribution threshold is competitive quality plus reproducible wall-clock
  gains at realistic operating points.

Related work:
- S4 [[gu2021-s4]]
- Hyena [[poli2023-hyena]]
- RWKV [[peng2023-rwkv]]
- RetNet [[sun2023-retnet]].

## 5. Minimum acceptance checklist for Goni backend candidates

Any architecture candidate should provide:

1. Scale realism
- trained/evaluated on non-toy corpora and practical model sizes.

2. Tokenizer realism
- reported quality with realistic subword tokenization.

3. Runtime realism
- prefill and decode measurements vs optimized transformer baselines at
  8k/32k/100k+ context ranges.

4. Memory realism
- peak memory/KV behavior and failure modes under long-context load.

5. Downstream usefulness
- not only perplexity; include instruction-following or task evaluations
  relevant to operator workflows.

6. Reproducibility
- open configs, ablations, and stable training behavior.

Related long-context evidence:
- HyenaDNA long-range sequence modeling at scale [[nguyen2023-hyenadna]]
- multimodal needle-in-haystack stress testing [[wang2024-mmneedle]].

## 6. Product implication for Goni

Goni should remain model-backend pluggable. Product trust should not depend on
any single architecture trend.

Practical stance:
- optimize memory architecture (retrieval, compaction, provenance),
- optimize mediation/governance (capabilities, receipts, policy),
- treat sequence-model innovation as a swap-in backend improvement path.

In deployment terms, long-context quality and cost are often constrained by
retrieval quality, KV behavior, and serving systems design as much as by model
family choice [[kwon2023-vllm]] [[wang2024-mmneedle]].

## 7. Position statement

For Goni architecture decisions:
- promising small-scale sub-quadratic results are tracked as R&D signals,
- production adoption requires realistic tokenizer + scale + wall-clock evidence
  against strong optimized transformer baselines.

## Upstream
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [ITCR](/blueprint/30-specs/itcr.md)
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)

## Downstream
- [Roadmap](/blueprint/docs/roadmap.md)
- [Metrics scorecard](/blueprint/docs/metrics.md)
- [Goni Lab](/blueprint/docs/goni-lab.md)

## Adjacent
- [Learning loop](/blueprint/20-system/50-learning-loop.md)
- [Agentic kernel foundations](/blueprint/20-system/40-agentic-kernel-foundations.md)
- [References](/blueprint/docs/references/README.md)
