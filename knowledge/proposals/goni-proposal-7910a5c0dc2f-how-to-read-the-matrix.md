---
id: GONI-PROPOSAL-7910A5C0DC2F
title: How to read the matrix
type: proposal
status: draft
implementation_state: specified_only
proposition: '**Weight placement matters:** a small VRAM number can hide a large system-RAM or storage requirement.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: How to read the matrix
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# How to read the matrix

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### How to read the matrix

- **Weight placement matters:** a small VRAM number can hide a large system-RAM
  or storage requirement. The physical location of the remaining weights is
  part of the hardware cost.
- **Capacity is not latency:** initialization, eventual token generation, and
  productive interactive response are three different thresholds. This matrix
  ranks for the third.
- **Serving is not offloading:** vLLM and SGLang improve execution and
  concurrency after a model has been provisioned. AirLLM, Accelerate, and
  FlexLLMGen instead explore increasingly aggressive memory hierarchies.
- **Operations matter:** installation, supported architectures, backend
  maturity, and configuration risk affect the reliability of a daily personal
  assistant as much as a peak benchmark does.

Four-bit arithmetic also sets a useful lower bound. Seventy billion weights at
four bits require `70,000,000,000 x 4 / 8 = 35,000,000,000` bytes, approximately
35 GB before quantization metadata, higher-precision tensors, KV cache, and
runtime buffers. A 24 GB GPU therefore requires CPU offload, multiple GPUs, a
lower effective bitrate, or a smaller model; ordinary 4-bit quantization alone
does not make a 70B model VRAM-resident on that card.

All engine codebases in the two tables use permissive MIT or Apache 2.0
licenses. The license for any selected model checkpoint is separate and must be
reviewed independently.
