# Local Models - Supplier Card

## What it is
- On-box local model engines and model families used for default local-first execution.

## Relevance to Goni
- Primary path for privacy and predictable baseline runtime behavior.
- Serves as fallback when remote council is unavailable or budget-limited.

## Notes for blueprint
- Record supported engines and model classes with test evidence.
- Separate benchmark evidence from strategy claims.
- Treat model servers as replaceable suppliers behind Goni routing, not as the
  control plane.

## Runtime supplier candidates

Source confidence uses the same labels as
[Adjacent Projects](/blueprint/docs/adjacent-projects.md): `verified`,
`needs verification`, `stale/deprecated`, and `candidate/unverified`.

| Candidate | Confidence | Routing relevance |
| --- | --- | --- |
| Ollama | `verified` | Simple local model UX and baseline OpenAI-like endpoint candidate. |
| llama.cpp / llama-server | `verified` | GGUF-first edge runtime and deterministic local baseline. |
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
| Hugging Face Transformers | `verified` | Research and model experimentation substrate. |
| LMDeploy | `verified` | Efficient deployment/serving candidate. |
| Xinference | `verified` | Higher-level model-serving platform wrapping multiple engines. |
| BitNet | `needs verification` | Low-bit inference direction; assess runtime maturity per model. |
| JittorInfer | `needs verification` | Domestic inference candidate; confirm official source and hardware support. |
| Xuanwu CLI / 玄武CLI | `needs verification` | Ollama-like domestic CLI/server candidate; confirm license and backend scope. |

## Routing notes

- Edge/APU/CPU-first path: prefer llama.cpp, Ollama-like UX, MLX/Apple paths,
  or Xinference-style backend wrappers.
- GPU-throughput path: evaluate vLLM, SGLang, TGI, TensorRT-LLM, and LMDeploy.
- Distributed-home path: track exo and the distributed-inference references in
  [Related Projects](/blueprint/docs/related-projects.md).
- Domestic/China ecosystem path: track Xuanwu CLI, KTransformers, JittorInfer,
  and Xinference, but do not promote them to defaults without license,
  hardware, and maintenance checks.

## Related docs
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Adjacent Projects](/blueprint/docs/adjacent-projects.md)
- [Goni Lab](/blueprint/docs/goni-lab.md)

