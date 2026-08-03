---
id: GONI-SYNTHESIS-C2C15E784C10
title: 6. llama.cpp – Single-node inference baseline
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Repository:** https://github.com/ggml-org/llama.cpp :contentReference[oaicite:29]{index=29} llama.cpp is the upstream project many of these systems build upon.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 6. llama.cpp – Single-node inference baseline
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 6. llama.cpp – Single-node inference baseline

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. llama.cpp – Single-node inference baseline

**Repository:**  
https://github.com/ggml-org/llama.cpp :contentReference[oaicite:29]{index=29}  

llama.cpp is the upstream project many of these systems build upon.

From the README:

> “The main goal of `llama.cpp` is to enable LLM inference with minimal setup and state-of-the-art performance on a wide range of hardware – locally and in the cloud.” :contentReference[oaicite:30]{index=30}  

Key features relevant here:

- Pure C/C++ implementation, MIT-licensed.  
- Supports:
  - Apple silicon (Metal), x86 with AVX/AVX2/AVX-512/AMX, RISC-V, and various GPU backends (CUDA, HIP, Vulkan, SYCL).:contentReference[oaicite:31]{index=31}  
- Implements many quantisation schemes (1.5–8 bits) to fit large models into modest memory.:contentReference[oaicite:32]{index=32}  
- Includes a **server mode** (`llama-server`) exposing an OpenAI-compatible HTTP API.:contentReference[oaicite:33]{index=33}  

EXO, prima.cpp, distributed-llama and others all use llama.cpp as either:

- the **baseline** for comparison, or  
- the **core inference engine** extended to distributed execution.

For Goni, llama.cpp (or an equivalent runtime like vLLM) is a natural choice for:

- single-node inference baseline,  
- potential multi-node experiments later.

---
