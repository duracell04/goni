---
id: GONI-PROPOSAL-7232A20EECA9
title: Runtime supplier candidates
type: proposal
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: Source confidence uses the same labels as Adjacent Projects: verified, needs verification, stale/deprecated, and candidate/unverified.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/suppliers/local-models.md
  heading: Runtime supplier candidates
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# Runtime supplier candidates

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Runtime supplier candidates

Source confidence uses the same labels as
[Adjacent Projects](/blueprint/docs/adjacent-projects.md): `verified`,
`needs verification`, `stale/deprecated`, and `candidate/unverified`.

| Candidate | Confidence | Routing relevance |
| --- | --- | --- |
| Ollama | `verified` | Simple local model UX and baseline OpenAI-like endpoint candidate. |
| llama.cpp / llama-server | `verified` | GGUF-first edge runtime and deterministic local baseline. |
| ExLlamaV3 | `verified` | NVIDIA-native quantized runtime for models and caches that fit substantially in aggregate VRAM. |
| ik_llama.cpp | `verified` | Experimental llama.cpp fork for specialized CPU, CUDA, quantization, and hybrid-MoE performance work. |
| Chitu | `verified` | Heterogeneous inference engine spanning CPU, GPU, hybrid, and distributed accelerator configurations. |
| LocalAI | `verified` | OpenAI-compatible local API with broader backend and multimodal ambitions. |
| LM Studio | `verified` | Desktop/server bridge for local model experimentation. |
| Jan local server | `verified` | Local-first app/server candidate for desktop-oriented deployments. |
| vLLM | `verified` | GPU throughput backend for stronger local or mesh nodes. |
| SGLang | `verified` | High-throughput structured-generation backend. |
| TensorRT-LLM | `verified` | NVIDIA-optimized serving path for high-end GPU nodes. |
| TGI / Hugging Face Text Generation Inference | `verified` | Production inference server in the Hugging Face ecosystem. |
| Mistral.rs | `verified` | Rust runtime candidate for local/server experiments. |
| exo | `verified` | Distributed home-device inference reference. |
| KTransformers | `verified` | Heterogeneous/MoE inference candidate. |
| Hugging Face Accelerate | `verified` | Generic GPU/CPU/disk model dispatcher for compatibility work and uncommon Transformers architectures. |
| FlexLLMGen | `verified` | Throughput-oriented GPU/CPU/storage offload reference for large offline jobs. |
| AirLLM | `verified` | Layer/expert streaming reference for minimum-allocation execution demonstrations, not interactive latency. |
| Hugging Face Transformers | `verified` | Research and model experimentation substrate. |
| LMDeploy | `verified` | Efficient deployment/serving candidate. |
| Xinference | `verified` | Higher-level model-serving platform wrapping multiple engines. |
| BitNet | `needs verification` | Low-bit inference direction; assess runtime maturity per model. |
| JittorInfer | `needs verification` | Domestic inference candidate; confirm official source and hardware support. |
| Xuanwu CLI / 玄武CLI | `needs verification` | Ollama-like domestic CLI/server candidate; confirm license and backend scope. |
