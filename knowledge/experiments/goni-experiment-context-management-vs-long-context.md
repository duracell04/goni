---
id: GONI-EXPERIMENT-CONTEXT-MANAGEMENT-VS-LONG-CONTEXT
title: Bounded Context Management Versus Raw Long Context
type: experiment
status: draft
implementation_state: not_applicable
proposition: >-
  Compare raw accumulating long context against bounded retrieval-based context
  and checkpoint/rollover strategies on matched local hardware to test whether
  GONI can preserve task quality while reducing context-serving resource cost.
domains:
- research
- context
- inference
relations:
- type: tests
  target: SOV-EFF-01
- type: tests
  target: CTX-LIFE-01
- type: tests
  target: TASK-CHECKPOINT-01
- type: tests
  target: ATTN-LOCAL-01
sources: []
artifacts: []
uncertainty: >-
  Experimental design only. Results may vary materially by model family,
  runtime, quantization, task distribution, retrieval quality, and hardware.
legacy: []
---

# Bounded Context Management Versus Raw Long Context

> Status boundary: this is an experiment definition, not evidence. No strategy is presumed superior before matched evaluation.

## Question

Does GONI obtain a better local system-level trade-off by carrying increasingly large raw context, or by maintaining a bounded active context backed by governed retrieval, structured checkpoints, and context rollover?

## Hypotheses

H1. For tasks whose relevant evidence is sparse relative to total history, bounded context plus governed retrieval will reduce memory use and prefill cost without materially reducing task success.

H2. Structured checkpoints plus fresh retrieval will preserve continuity across rollover better than unconstrained prose summarization alone.

H3. Reducing per-task active context will increase the number of concurrent resident or cheaply resumable WorkOrders that fit inside a fixed local hardware envelope.

H4. Efficient-attention runtimes may shift the crossover point at which raw long context becomes competitive, so context policy should be model/runtime-aware rather than fixed globally.

## Experimental arms

Use synthetic, domain-neutral workloads and the same hardware, model quality class, tool permissions, and evaluation budget where possible.

### Arm A: accumulating raw context

Carry the full available task transcript until the model or runtime context ceiling is reached.

### Arm B: bounded context plus retrieval

Maintain a fixed or policy-bounded active context. Retrieve prior evidence through the canonical dense, sparse, metadata, and graph-aware retrieval path.

### Arm C: bounded context plus retrieval and checkpoint rollover

Use Arm B plus `TASK-CHECKPOINT-01` to create structured continuation state when context pressure triggers rollover, then rehydrate a fresh `ContextPack`.

### Arm D: efficient-attention long-context runtime

Repeat the most relevant long-context strategy on a runtime/model with materially different KV/attention economics, where a suitable open-weight implementation is available. This arm tests whether architectural KV/attention improvements alter the system-level optimum rather than assuming they do.

## Workloads

The benchmark set SHOULD include:

- multi-turn factual tasks with early evidence needed late,
- long-document synthesis,
- coding or planning tasks with intermediate decisions,
- tasks containing corrected or superseded facts,
- tasks requiring exact source recovery,
- interruptions followed by later resumption, and
- several concurrent WorkOrders competing for local memory.

Workloads must avoid relying on private user history so the experiment is reproducible.

## Primary quality metrics

- task success rate,
- grounded-answer accuracy,
- exact-source recovery where required,
- contradiction and correction preservation,
- decision continuity after rollover,
- checkpoint reconstruction success,
- retrieval miss rate,
- stale-state error rate, and
- unauthorized or policy-inconsistent state carryover.

## Resource metrics

- time to first token,
- prefill latency,
- decode latency or time per output token,
- tokens per second where useful,
- peak RAM/VRAM/unified-memory occupancy,
- KV-cache bytes versus active context,
- memory-bandwidth utilization where measurable,
- joules per completed task,
- thermal throttling behavior, and
- storage/replay overhead.

## Resident cognitive concurrency

A specific GONI metric is the number of useful task states the appliance can keep resident or resume within an acceptable latency bound:

[
N_{resident}
approx
rac{
M_{available} - M_{weights} - M_{system}
}{
M_{active task}
}.
]

This formula is only an approximation because fragmentation, shared prefixes, batching, heterogeneous task sizes, and runtime scheduling affect actual capacity. The measured metric should therefore report both concurrent resident WorkOrders and resume latency after eviction.

## Acceptance criteria

The experiment should not declare a winner from one aggregate score. Instead, report the Pareto frontier across task quality, latency, memory, energy, and concurrency.

Evidence would support `SOV-EFF-01` if bounded context management produces comparable or better task utility at meaningfully lower local resource cost for at least one representative workload class without increasing policy or provenance failures.

Evidence would challenge the thesis if raw long context consistently dominates bounded strategies under matched resource constraints, or if checkpoint/retrieval failures erase the expected systems gains.

## Reproducibility

Record:

- model and exact checkpoint hash,
- runtime and version,
- quantization,
- hardware and driver versions,
- context policy and thresholds,
- retrieval/index versions,
- checkpoint schema version,
- prompt/tool schemas,
- seeds where meaningful, and
- raw benchmark outputs.

Results should be submitted as evidence only after the experiment pins the implementing repository and commit and states the tested boundary.
