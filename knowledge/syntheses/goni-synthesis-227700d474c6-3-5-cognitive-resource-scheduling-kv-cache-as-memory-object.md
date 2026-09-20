---
id: GONI-SYNTHESIS-227700D474C6
title: 3.5 Cognitive resource scheduling (KV cache as memory object)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Interactive agent scheduling must include LLM-serving memory realities: KV cache residency, representation size, fragmentation, bandwidth, admission, and eviction affect tail latency and should be evaluated as explicit scheduling variables under mixed workloads.'
domains:
- agent
- kernel
- system
aliases: []
relations:
- type: depends_on
  target: GONI-PRINCIPLE-9062425CD490
- type: synthesizes
  target: GONI-EVIDENCE-6F21B6FCEF3C
- type: synthesizes
  target: GONI-EVIDENCE-55CFEAFCBF16
sources:
- SRC-KWON2023-VLLM
- SRC-ZHANG2023-H2O
- SRC-DEEPSEEK2024-V2
artifacts: []
uncertainty: PagedAttention, heavy-hitter retention, and MLA address different parts of the KV problem. Their reported benefits do not establish one universal Goni cache policy; model, workload, hardware, and backend effects must be measured independently.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 3.5 Cognitive resource scheduling (KV cache as memory object)
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.5 Cognitive resource scheduling (KV cache as memory object)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 Cognitive resource scheduling (KV cache as memory object)

Interactive agent scheduling must include LLM-serving memory realities:

- KV-cache residency and fragmentation affect tail latency;
- scheduler decisions should include memory residency and bandwidth;
- admission and eviction policy must be explicit under mixed workloads;
- representation size and sequence retention are separate optimization axes;
- irregular sparsity must be evaluated against hardware locality, batching, and
  kernel overhead.

## Three distinct KV optimization axes

Goni should keep three questions separate.

### 1. Residency and paging

PagedAttention demonstrates that KV memory management and fragmentation can
materially affect serving throughput. This supports treating cache residency as
a first-class scheduler concern rather than an opaque runtime detail.
[[kwon2023-vllm]]

### 2. Sequence selection

H2O reports heavy-hitter behavior in cumulative attention and proposes retaining
important historical tokens together with recent tokens under a constrained
cache. For Goni, the transferable research question is whether future utility
is sufficiently concentrated to justify non-uniform retention.
[[zhang2023-h2o]]

Heavy-hitter observations MUST NOT be translated into lexical-frequency rules.
A token identity does not define one reusable KV state; KV vectors remain
context-dependent.

A cache policy may compare signals such as:

- recency;
- cumulative attention or another observed-use signal;
- estimated future reuse;
- semantic or task importance;
- reconstruction cost;
- storage and bandwidth cost.

Heavy-hitter retention is a baseline to evaluate, not a universal invariant.
Long-range dependencies can reactivate previously cold states, so aggressive
retention policies need recall-oriented candidate mechanisms, protected recent
state, or another measured recovery path.

### 3. Representation compression

DeepSeek-V2's Multi-head Latent Attention demonstrates an architecture-specific
method for reducing how many bytes of KV representation must be retained.
[[deepseek2024-v2]]

This is complementary to sequence selection:

[
	ext{KV cost}
approx
	ext{surviving sequence states}
	imes
	ext{representation bytes per state}
	imes
	ext{layer residency}.
]

A runtime may improve one axis without changing the others. Goni SHOULD measure
them separately so a gain from latent representation compression is not
mistaken for a gain from eviction, and vice versa.

## Variable fidelity as a research direction

Where runtimes permit multiple precisions or representations, Goni may test
whether higher-value states deserve higher-fidelity storage while lower-value
states use compressed forms.

A conceptual allocation rule is:

[
bits_i
propto
P(	ext{future use}_i)
	imes
Importance_i.
]

This frames KV optimization as a rate-distortion problem: reduce memory and
bandwidth while bounding degradation in output quality. The formula is a
research scaffold, not a prescribed bit-allocation algorithm.

## Hardware-aware constraint

A theoretically sparse policy can lose in practice if candidate selection,
random memory access, synchronization, decompression, or small irregular
kernels cost more than the attention work avoided.

Therefore evaluation SHOULD report end-to-end effects on:

- TTFT and decode latency;
- throughput;
- bytes moved from accelerator memory;
- peak KV memory;
- GPU or accelerator occupancy where observable;
- energy or thermal proxy;
- output quality and long-range recall.

Block-level selection, contiguous layouts, and discrete cache budgets may be
preferable to finer-grained policies when they preserve batching and predictable
kernels.

## Goni boundary

KV state is cognitive working state. Approximate retention or compression may
change what the model can recall during reasoning, but it MUST NOT become the
sole source of kernel authority for mandates, capabilities, policy, approval,
or revocation.

Related foundations:

- PagedAttention and memory-management analysis in vLLM [[kwon2023-vllm]];
- H2O heavy-hitter KV-cache retention [[zhang2023-h2o]];
- DeepSeek-V2 Multi-head Latent Attention [[deepseek2024-v2]].
