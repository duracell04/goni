---
id: "EVID-SOUP2026-LOWVRAM"
title: "Soup: low-VRAM LoRA/QLoRA training through layer streaming"
type: "evidence"
status: "draft"
implementation_state: "not_applicable"
proposition: "Soup's pinned upstream records report that streaming frozen, quantized transformer layers from host memory permits a specific LoRA fine-tuning run of an 8B model on a 4 GB NVIDIA laptop GPU; applicability to GONI hardware and total-system economics remain unmeasured."
domains:
- "hardware"
- "research"
- "software"
aliases: []
relations:
- type: "supports"
  target: "GONI-PRINCIPLE-9062425CD490"
sources:
- "SRC-SOUP-REPO"
- "SRC-SOUP-BENCHMARKS"
artifacts: []
uncertainty: "The 4 GB run and its correctness/performance measurements are reported by Soup's project, not independently reproduced here. Results are configuration- and version-dependent; unified-memory APU compatibility and economic superiority over simpler alternatives are open questions."
legacy: []
---

# Soup: low-VRAM LoRA/QLoRA training through layer streaming

## Reported upstream observations

Soup provides YAML-driven dataset preparation, fine-tuning, evaluation, and export. Its optional `training.stream_layers: true` path keeps the frozen base model in host RAM or a supported disk-backed store and transfers decoder layers into reusable accelerator buffers during training. LoRA adapters, their gradients, and optimizer states remain trainable. The base is still needed for gradient propagation and is read again during the backward computation. This is **parameter-efficient fine-tuning**, not an 8B-parameter full-model training result.

The upstream repository reports an RTX 3050 Laptop GPU (4 GB) run of Llama-3.1-8B-Instruct with NF4, LoRA, batch size 1, and sequence length 512. The README reports a pre-correctness-repair measurement of 3.32 GB peak VRAM at 119.6 training tokens/s (952 MHz); its later 2.40 GB / 208.6 tokens/s observation used a different RTX 3050 and higher GPU clocks, and is expressly not a like-for-like speedup. The benchmark gate reports bit-exact comparisons with *resident NF4 computations under the tested configuration*, not equivalence to full-precision training. The tests and record also document repaired adapter-path and correctness defects.

The [pinned layer-streaming sizing documentation](https://github.com/MakazhanAlpamys/Soup/blob/4cff43bf967e96a484381205e5125e2fc39227f1/docs/performance-and-quantization.md) explains that activation and vocabulary-logit memory still grow with batch size and sequence length. Its example estimates roughly 3.448 GB at 512 tokens and 4.512 GB at 1,024 tokens, exceeding the stated card capacity before further memory-saving provisions.

The [controlled timing probe](https://github.com/MakazhanAlpamys/Soup/blob/4cff43bf967e96a484381205e5125e2fc39227f1/benchmarks/probe-v0.73.0-what-bounds-streaming.md) reports 1.44% less step time when all host-to-device transfers were suppressed, versus 9.80% when repeated NF4 dequantization was suppressed. These are **timing-only, mathematically invalid ablations** used to isolate bottlenecks, not executable training optimizations or general performance guarantees.

## GONI interpretation

Soup is a candidate **offline adaptation backend** for recurrent, measurable semantic tasks. It belongs outside the kernel's authority boundary: adapted weights cannot grant mandates, tool permissions, trusted memory, or execution rights. Training data, adapter revisions, validation results, and deployment approvals require explicit provenance and GONI-governed promotion.

This evidence complements [LoRA](evid-hu2022-lora.md), [AirLLM's inference streaming](evid-airllm-layer-expert-streaming.md), [FlexGen's inference offloading](evid-sheng2023-flexgen-offloading.md), and [FrugalGPT-style inference routing](goni-evidence-25f334db139d-source-claim-frugalgpt2023.md). Each addresses a different resource or lifecycle stage. Training feasibility alone does not establish lower total cost, better task quality, or beneficial continuous on-device training.

The headline result uses a discrete NVIDIA CUDA GPU plus host resources. GONI's APU/unified-memory platform must be evaluated separately, including runtime support, shared-memory pressure, storage traffic, energy, thermals, and opportunity cost of competing interactive work.

## Evidence boundary

This node records **external project-reported feasibility and known limitations**. It does not assert that GONI runs Soup, that the benchmark has been reproduced, or that model specialization outperforms retrieval, small resident models, deterministic code, or a larger reference model. [EXP-ADAPT-01](../experiments/exp-adapt-01-local-model-specialization.md) specifies the decision experiment.
