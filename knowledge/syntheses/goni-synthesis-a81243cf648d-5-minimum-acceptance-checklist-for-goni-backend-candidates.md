---
id: GONI-SYNTHESIS-A81243CF648D
title: 5. Minimum acceptance checklist for Goni backend candidates
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Any architecture candidate should provide: Scale realism trained/evaluated on non-toy corpora and practical model sizes.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: 5. Minimum acceptance checklist for Goni backend candidates
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# 5. Minimum acceptance checklist for Goni backend candidates

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
