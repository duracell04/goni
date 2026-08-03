---
id: GONI-PROPOSAL-81C9BC383209
title: Routing notes
type: proposal
status: draft
implementation_state: specified_only
proposition: 'Edge/APU/CPU-first path: prefer llama.cpp, Ollama-like UX, MLX/Apple paths, or Xinference-style backend wrappers.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/suppliers/local-models.md
  heading: Routing notes
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# Routing notes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Routing notes

- Edge/APU/CPU-first path: prefer llama.cpp, Ollama-like UX, MLX/Apple paths,
  or Xinference-style backend wrappers.
- GPU-throughput path: evaluate vLLM, SGLang, TGI, TensorRT-LLM, and LMDeploy.
- Distributed-home path: track exo and the distributed-inference references in
  [Related Projects](/blueprint/docs/related-projects.md).
- Domestic/China ecosystem path: track Xuanwu CLI, KTransformers, JittorInfer,
  and Xinference, but do not promote them to defaults without license,
  hardware, and maintenance checks.
