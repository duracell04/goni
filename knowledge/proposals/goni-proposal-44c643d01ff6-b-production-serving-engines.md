---
id: GONI-PROPOSAL-44C643D01FF6
title: B. Production-serving engines
type: proposal
status: draft
implementation_state: specified_only
proposition: These engines solve a different problem and therefore remain outside the constrained-hardware ranking.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: B. Production-serving engines
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# B. Production-serving engines

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### B. Production-serving engines

These engines solve a different problem and therefore remain outside the
constrained-hardware ranking.

| Project | License | Primary optimization | Hardware assumption | Best use | Why it is not an AirLLM replacement |
| --- | --- | --- | --- | --- | --- |
| [vLLM](https://docs.vllm.ai/) | Apache 2.0 | Continuous batching, cache management, tensor parallelism, pipeline parallelism, and expert parallelism | The model is sensibly distributed across sufficient accelerator memory | High-throughput OpenAI-compatible serving, multi-user systems, and large production deployments | It optimizes execution after weights have been properly placed; it does not primarily turn a tiny GPU into a practical host for a trillion-parameter checkpoint. |
| [SGLang](https://docs.sglang.ai/) | Apache 2.0 | RadixAttention, prefix caching, structured generation, speculative decoding, and distributed serving | One adequately sized GPU through large clusters | Agent systems, repeated prefixes, structured outputs, and production inference | Its strengths are latency and throughput at serving scale, rather than continuous disk streaming under severe VRAM constraints. |
