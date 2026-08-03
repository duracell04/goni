---
id: GONI-PROPOSAL-4D6051A4CC18
title: Kimi K3 status on 2026-08-02
type: proposal
status: draft
implementation_state: specified_only
proposition: '| Runtime | Snapshot status | Practical interpretation | | AirLLM | Publicly demonstrated with 3.72 GB peak VRAM on one RTX 6000 Ada through per-expert streaming | Lowest documented GPU allocation in this comparison, but primarily an execution demonstration rather than a fast assistant path.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Kimi K3 status on 2026-08-02
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Kimi K3 status on 2026-08-02

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Kimi K3 status on 2026-08-02

| Runtime | Snapshot status | Practical interpretation |
| --- | --- | --- |
| [AirLLM](https://github.com/lyogavin/airllm) | Publicly demonstrated with 3.72 GB peak VRAM on one RTX 6000 Ada through per-expert streaming | Lowest documented GPU allocation in this comparison, but primarily an execution demonstration rather than a fast assistant path. |
| [vLLM](https://vllm-project.github.io/2026/07/27/k3.html) | Live K3 support includes multimodal processing, tool calling, reasoning output, structured output, and production deployment recipes | Strongest current production path, provided substantial accelerator infrastructure is available. |
| [llama.cpp](https://github.com/ggml-org/llama.cpp/pull/26185) | Text-model support remains an open, unmerged upstream pull request; a full-size multimodal fork reports functional layer splitting while row and tensor splitting remain requested | Promising for GGUF and hybrid deployment, but too fresh and fork-dependent to call stable. See the current [split-mode request](https://github.com/ggml-org/llama.cpp/issues/26365). |
| [KTransformers](https://github.com/kvcache-ai/ktransformers/issues/2109) | No released K3 support was evident; the support request opened on 2026-07-28 remains unresolved | Potentially interesting for a RAM-heavy workstation, but not K3-ready at this snapshot. |
