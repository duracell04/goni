---
id: GONI-IMAP-EE2B5AB9F80D
title: 3.2 Backend abstraction
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Concrete engines (llama.cpp, vLLM, etc.) implement LlmRuntime: MVP: 1–2 backends is enough (e.g.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 3.2 Backend abstraction
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 3.2 Backend abstraction

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Backend abstraction

Concrete engines (llama.cpp, vLLM, etc.) implement LlmRuntime:

* MVP: 1–2 backends is enough (e.g. local small/large model).

Backends may optionally expose speculative decoding with a draft model or
attached draft heads. vLLM documents this draft/verify path, while DSpark is a
production example of semi-autoregressive drafting plus confidence-scheduled
verification; DeepSpec is the related DeepSeek training/evaluation codebase for
speculative drafters. [[vllm-speculative-decoding]] [[cheng2026-dspark]] [[deepseek2026-deepspec]]

Runtime candidates are tracked as technology intelligence in
[Local Models](/blueprint/60-market/suppliers/local-models.md) and
[Adjacent Projects](/blueprint/docs/adjacent-projects.md). Examples include
llama.cpp, Ollama, LocalAI, LM Studio, Jan local server, vLLM, SGLang, TGI,
TensorRT-LLM, LMDeploy, Xinference, KTransformers, Mistral.rs, and exo.
These are backend choices behind this interface; none owns routing policy,
receipts, or capability mediation.

---
