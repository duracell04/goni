---
id: GONI-PROPOSAL-9A4087400064
title: A. Local and constrained-hardware runtimes
type: proposal
status: draft
implementation_state: specified_only
proposition: '| Priority | Project | Runtime class | Where the model weights live | Hardware sweet spot | Latency profile | Setup burden | Best use | Decisive limitation | | 1 | llama.cpp | General local runtime | GGUF weights distributed between VRAM and system RAM through partial GPU offload and memory mapping | CPUs, Apple Silicon, NVIDIA, AMD, and Intel; especially 4-24 GB GPUs with adequate RAM | Interactive when most computation remains on GPU; progressively slower as more layers remain on CPU | Low to'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: A. Local and constrained-hardware runtimes
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# A. Local and constrained-hardware runtimes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### A. Local and constrained-hardware runtimes

| Priority | Project | Runtime class | Where the model weights live | Hardware sweet spot | Latency profile | Setup burden | Best use | Decisive limitation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | [llama.cpp](https://github.com/ggml-org/llama.cpp) | General local runtime | GGUF weights distributed between VRAM and system RAM through partial GPU offload and memory mapping | CPUs, Apple Silicon, NVIDIA, AMD, and Intel; especially 4-24 GB GPUs with adequate RAM | Interactive when most computation remains on GPU; progressively slower as more layers remain on CPU | Low to medium | Default local assistant, broad hardware support, experimentation, and reliable deployment | Extremely large models still require enough aggregate RAM and become CPU-bandwidth constrained when heavily offloaded. |
| 2 | [ExLlamaV3](https://github.com/turboderp-org/exllamav3) | GPU-native quantized runtime | EXL3 weights reside substantially in aggregate GPU memory, with tensor and expert parallelism across GPUs | Modern NVIDIA GPUs, especially one or several 24 GB cards | Excellent when the model fits in VRAM | Medium | Maximum interactive speed, long conversations, and multi-user local agents on NVIDIA | It is not primarily a CPU or NVMe spillover system; performance depends on fitting the quantized model and cache substantially inside available VRAM. |
| 3 | [KTransformers](https://github.com/kvcache-ai/ktransformers) | Heterogeneous MoE runtime | Attention and selected components use the GPU; experts can remain in system RAM and execute through optimized CPU kernels and scheduling | One strong GPU plus 128-512 GB of fast RAM, preferably modern DDR5 with high memory bandwidth | Conditionally interactive, depending heavily on model, CPU, and RAM bandwidth | High | Very large sparse MoE models that exceed GPU memory by a wide margin | Architecture support is model-specific, installation is demanding, and memory bandwidth becomes the principal bottleneck. |
| 4 | [ik_llama.cpp](https://github.com/ikawrakow/ik_llama.cpp) | Experimental high-performance llama.cpp fork | GGUF weights split selectively between CPU and CUDA, with specialized MoE kernels, tensor overrides, and additional quantization formats | Technical users with strong AVX2/AVX-512 CPUs and NVIDIA CUDA GPUs | Good to conditional, particularly for supported MoE workloads | High | Additional CPU or hybrid-MoE performance beyond mainline llama.cpp | The fork diverges from mainline, and its documentation identifies CPU and CUDA as the fully functional, performant backends; compatibility and configuration risk are higher. |
| 5 | [Chitu](https://github.com/thu-pacman/chitu) | Heterogeneous enterprise inference engine | Supports CPU, single-GPU, CPU-GPU hybrid, and distributed accelerator configurations, including FP4 and FP8 conversion paths | NVIDIA or Chinese accelerator infrastructure, from one card to multi-node deployments | Conditional to production-grade, depending on configuration | High | Large MoE deployment where heterogeneous hardware and later scaling matter | It is more operationally complex than desktop-focused runtimes, with a smaller international user ecosystem and more platform-specific deployment paths. |
| 6 | [Hugging Face Accelerate](https://huggingface.co/docs/accelerate/concept_guides/big_model_inference) | Generic model dispatcher and offloader | Modules can be allocated across GPU, CPU, and memory-mapped disk through an explicit or automatic device map | Researchers supporting uncommon Transformers architectures | Usually slow under heavy offload | Medium to high | Compatibility experiments, custom architectures, and implementation work | Multi-GPU model parallelism is deliberately basic and can leave GPUs operating sequentially; Accelerate is a compatibility layer, not a highly optimized local inference engine. |
| 7 | [FlexLLMGen](https://github.com/FMInference/FlexLLMGen) | Throughput-oriented offload engine | GPU, CPU, and storage are jointly scheduled, using compression and large effective batches | One commodity GPU running large offline jobs | Batch-oriented, not conversational | High | Overnight extraction, benchmarking, data processing, and high-volume offline inference | Its optimization target is aggregate throughput over long jobs rather than low first-token latency or interactive chat. |
| 8 | [AirLLM](https://github.com/lyogavin/airllm) | Layer-wise or expert-wise streaming runtime | Individual layers or routed experts are repeatedly loaded from storage into GPU memory | Small VRAM, very large SSD capacity, and workloads where execution matters more than response time | Proof-of-execution for extreme models | Medium | Architecture inspection, compatibility testing, and demonstrating that a checkpoint can technically execute | It minimizes peak VRAM by transferring the bottleneck to storage traffic, weight movement, and token latency. Its Kimi K3 demonstration used 3.72 GB peak allocation on an RTX 6000 Ada, not an actual 4 GB card. |
