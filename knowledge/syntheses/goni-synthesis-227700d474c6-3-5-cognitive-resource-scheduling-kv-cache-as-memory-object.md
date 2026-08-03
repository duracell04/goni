---
id: GONI-SYNTHESIS-227700D474C6
title: 3.5 Cognitive resource scheduling (KV cache as memory object)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Interactive agent scheduling must include LLM-serving memory realities: KV cache residency and fragmentation affect tail latency, scheduler decisions should include memory residency and bandwidth, eviction/admission policy must be explicit under mixed workloads.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 3.5 Cognitive resource scheduling (KV cache as memory object)
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.5 Cognitive resource scheduling (KV cache as memory object)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 Cognitive resource scheduling (KV cache as memory object)

Interactive agent scheduling must include LLM-serving memory realities:
- KV cache residency and fragmentation affect tail latency,
- scheduler decisions should include memory residency and bandwidth,
- eviction/admission policy must be explicit under mixed workloads.

Related foundations:
- PagedAttention and memory-management analysis in vLLM
  [[kwon2023-vllm]].
