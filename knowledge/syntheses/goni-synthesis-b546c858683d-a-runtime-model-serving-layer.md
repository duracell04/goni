---
id: GONI-SYNTHESIS-B546C858683D
title: A) Runtime / model serving layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: These are backends for llm-runtime or model routing.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: A) Runtime / model serving layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# A) Runtime / model serving layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### A) Runtime / model serving layer

These are backends for `llm-runtime` or model routing. None replaces Goni's
kernel, policy, receipt, or memory governance.

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| Ollama | `verified` | Simple local model runtime and model manager. |
| llama.cpp / llama-server | `verified` | Low-level GGUF inference baseline and OpenAI-compatible local server path. |
| ExLlamaV3 | `verified` | NVIDIA-native quantized runtime for models and caches that fit substantially in aggregate VRAM. |
| ik_llama.cpp | `verified` | Experimental llama.cpp fork for specialized CPU, CUDA, quantization, and hybrid-MoE performance work. |
| Chitu | `verified` | Heterogeneous inference engine spanning CPU, GPU, hybrid, and distributed accelerator configurations. |
| LocalAI | `verified` | Local OpenAI-compatible API with broad backend and multimodal ambitions. |
| LM Studio | `verified` | Desktop runtime plus OpenAI-compatible local server. |
| Jan local server | `verified` | Local-first desktop app that can also expose local model serving. |
| vLLM | `verified` | Production-grade GPU serving and batching path for stronger nodes. |
| SGLang | `verified` | Serving/runtime layer for structured generation and high-throughput inference. |
| TensorRT-LLM | `verified` | NVIDIA-optimized high-throughput serving path. |
| TGI / Hugging Face Text Generation Inference | `verified` | Hugging Face production inference server, useful for GPU-backed deployments. |
| Mistral.rs | `verified` | Rust inference engine candidate for local and server use. |
| exo | `verified` | Everyday-device clustering and distributed local inference reference. |
| BitNet | `needs verification` | 1-bit inference/model-family direction; evaluate by concrete runtime maturity. |
| KTransformers | `verified` | Heterogeneous/MoE-optimized inference candidate, especially CPU+GPU mixes. |
| Hugging Face Accelerate | `verified` | Generic GPU/CPU/disk model dispatcher for compatibility work and uncommon Transformers architectures. |
| FlexLLMGen | `verified` | Throughput-oriented GPU/CPU/storage offload reference for large offline jobs. |
| AirLLM | `verified` | Layer/expert streaming reference for minimum-allocation execution demonstrations, not interactive latency. |
| Hugging Face Transformers | `verified` | Research and experimentation baseline, not usually the appliance serving layer. |
| LMDeploy | `verified` | Efficient LLM deployment/serving framework candidate. |
| JittorInfer | `needs verification` | China-origin inference candidate tied to Jittor/Ascend-style deployment notes. |
| Xinference | `verified` | Model-serving platform that can wrap vLLM, SGLang, llama.cpp, MLX, and others. |
| Xuanwu CLI / 玄武CLI | `needs verification` | Ollama-like domestic CLI/server candidate; confirm license and backend maturity. |

Runtime conclusion for Goni:

- Edge and consumer nodes should bias toward llama.cpp, Ollama-like UX,
  MLX/Apple paths, or Xinference-style wrappers.
- GPU servers should evaluate vLLM, SGLang, TGI, TensorRT-LLM, and LMDeploy.
- Goni's model router should treat all of these as swappable runtime targets
  behind policy, receipts, budgets, and local/cloud routing.
