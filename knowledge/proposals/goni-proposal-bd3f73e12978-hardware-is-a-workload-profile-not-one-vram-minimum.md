---
id: GONI-PROPOSAL-BD3F73E12978
title: Hardware is a workload profile, not one VRAM minimum
type: proposal
status: draft
implementation_state: specified_only
proposition: '"At least 8 GB VRAM" is not a universal requirement.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Hardware is a workload profile, not one VRAM minimum
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Hardware is a workload profile, not one VRAM minimum

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Hardware is a workload profile, not one VRAM minimum

"At least 8 GB VRAM" is not a universal requirement. Memory use depends on
parameter count, quantization, context length and KV cache, concurrency,
multimodal encoders, and how much work is offloaded to a GPU. Runtimes such as
llama.cpp can use CPU inference and CPU/GPU hybrid offload for models that do
not fit entirely in VRAM.

| Deployment profile | Practical model envelope | Expected trade-off |
| --- | --- | --- |
| Low-VRAM developer machine | Small 2B-4B quantized models, or larger 7B-9B-class models split across GPU and system RAM | Useful for plumbing and tool-loop tests; lower speed, context, or quality |
| 8-16 GB discrete GPU | Many 7B-14B-class quantized instruction models | Responsive single-user assistant, with context and multimodal limits determined by the bundle |
| 24-48 GB accelerator memory | Larger 14B-32B-class quantized models and more resident context | Better reasoning and tool reliability at higher power and cost |
| 64-128 GB unified memory | Larger quantized models, including the 30B-40B class targeted by the Goni reference design | Broad local coverage; bandwidth, thermal limits, and time-to-first-token still matter |

These are evaluation bands, not guarantees. Every promoted model bundle must
be measured on the target backend and hardware. Model weights fitting in
memory does not guarantee acceptable latency or enough room for KV cache,
embeddings, OCR, and concurrent system services.
