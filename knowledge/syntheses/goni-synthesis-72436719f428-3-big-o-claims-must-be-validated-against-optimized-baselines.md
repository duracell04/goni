---
id: GONI-SYNTHESIS-72436719F428
title: 3. Big-O claims must be validated against optimized baselines
type: synthesis
status: draft
implementation_state: specified_only
proposition: Asymptotic complexity alone is insufficient.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: 3. Big-O claims must be validated against optimized baselines
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# 3. Big-O claims must be validated against optimized baselines

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
