---
id: GONI-SYNTHESIS-0222756C2155
title: H) Hardware, edge, and deployment substrate
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: | Project / platform | Confidence | Goni relevance | | NVIDIA CUDA / TensorRT | verified | GPU acceleration and production throughput path.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: H) Hardware, edge, and deployment substrate
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# H) Hardware, edge, and deployment substrate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### H) Hardware, edge, and deployment substrate

| Project / platform | Confidence | Goni relevance |
| --- | --- | --- |
| NVIDIA CUDA / TensorRT | `verified` | GPU acceleration and production throughput path. |
| Apple Silicon / MLX | `verified` | Local Apple-device inference path and edge development reference. |
| Intel NUC | `verified` | Small-form-factor node class. |
| Beelink | `verified` | Mini-PC node class. |
| Coral TPU | `verified` | Edge accelerator path for small models/signals. |
| ESP32-S3 | `verified` | Voice satellite microcontroller class. |
| Off Grid-style phone stacks | `candidate/unverified` | Phone-local AI pattern; needs concrete project/source before use. |
| exo clustering | `verified` | Home/edge device clustering reference, also listed under runtime. |

Hardware conclusion:

- The physical substrate determines which runtime candidates are realistic.
- vLLM/SGLang/TensorRT-LLM are GPU-server oriented.
- llama.cpp, MLX, Ollama-like UX, and Xinference-style wrappers are more
  plausible on edge, APU, CPU, or desktop-class nodes.
