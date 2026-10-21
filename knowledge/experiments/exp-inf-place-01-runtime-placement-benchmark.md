---
id: EXP-INF-PLACE-01
title: Inference placement feasibility and efficiency benchmark
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Measure whether approved resident, offloaded, layer-streamed, and expert-streamed inference profiles can satisfy named Goni jobs within hardware, latency, quality, and resource budgets."
domains:
- hardware
- research
- software
aliases: []
relations:
- type: tests
  target: INF-PLACE-01
sources:
- SRC-AIRLLM-REPO
- SRC-SHENG2023-FLEXGEN
artifacts: []
uncertainty: "Runtime compatibility and performance are unmeasured on Goni target hardware; this is an experiment specification rather than benchmark evidence."
legacy: []
---

# Inference placement feasibility and efficiency benchmark

## Research question

On identified Goni hardware and representative workloads, when does
memory-hierarchy-aware inference provide useful local capabilities beyond
resident execution, and when do transfer costs defeat its benefit?

The hypothesis is conditional: streaming can reduce peak accelerator memory
for suitable supported model/runtime combinations, while latency, storage
traffic, total memory demand, and energy can make other execution modes
preferable.

## Controlled comparisons

For each supported model and precision profile, compare:

1. Resident weights when the model and context fit.
2. Conventional accelerator/CPU offloading.
3. Layer-wise disk or RAM streaming.
4. Routed-expert streaming for compatible sparse-MoE architectures.

Retain a separate **product baseline** using the best available smaller or
quantized resident model for the same task. Precision- and model-dependent
quality differences are measured explicitly; placement-only comparisons use
the same checkpoint, tokenizer, numerical precision, prompts, and output
budgets where backend capabilities permit.

Report results separately for unified-memory APU and discrete-GPU
configurations. Each result identifies hardware, OS, driver, runtime version,
checkpoint hash, precision, context length, batch size, and storage medium.

## Measurement protocol

Start with a small synthetic smoke test covering supported modes, adapter
loading, cancellation, and budget reporting. Advance to representative larger
models only after stability and resource estimates are confirmed. Bound test
runs by declared wall-time, storage traffic, energy, and thermal budgets.

Measure cold-start and warmed execution; short and long prompts; short and
long decode; interactive single-job and queued background work. Use repeated
runs and report sample count, variation, failures, and measurement uncertainty.

Collect:

- execution feasibility, required disk capacity, preparation time, and errors;
- time to first token, prefill duration, decode tokens/s, total completion time,
  and p50/p95 latency where repeated samples support quantiles;
- peak accelerator RAM, system RAM, KV/activation footprint, and concurrency
  headroom;
- storage bytes read and written, observed throughput, and write amplification
  where observable;
- energy in joules and thermal behavior where instrumented; measured proxies
  receive a distinct label;
- admission estimate error, cancellation responsiveness, and scheduler
  backpressure/interference with an interactive workload;
- task accuracy, evidence fidelity, and accepted-output rate on a fixed,
  predeclared evaluation set.

Record source-measured metrics separately from computed estimates. Treat
project-reported minimum-VRAM figures as external feasibility claims.

## Evaluation and promotion

Before measurements, define the task-specific minimum quality, response-time
or completion deadline, memory ceiling, storage traffic allowance, and
measurement coverage required for acceptance. Compare each candidate against
the resident/offloaded alternatives and the practical small-model baseline.

A placement mode is eligible for a production runtime profile only when
repeatable tests establish supported bundle/hardware combinations, resource
accounting, acceptable user-visible quality, and the workload's declared SLO.
Publish negative results, unsupported paths, and transfer bottlenecks alongside
successful configurations. Retain measurements as evidence with pinned
hardware, software, and model identifiers.

The executable harness and measurement artifacts belong in
`duracell04/goni-prototype-lab`; this Blueprint node specifies the experiment
and its promotion boundary.
