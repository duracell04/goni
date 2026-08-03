---
id: GONI-SYNTHESIS-1E3EB9EADED0
title: 2.1 Use-case constraints
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Local-first**: 80-90% of tokens should be served by **local models** on Goni.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 2.1 Use-case constraints
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 2.1 Use-case constraints

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Use-case constraints

- **Local-first**: 80-90% of tokens should be served by **local models** on Goni.  
- **Cloud-as-needed**: [LLM Council](/blueprint/docs/llm-council.md) used only when:
  - explicitly requested by user, or
  - orchestrator deems task "high difficulty" or requires long context.
  - Remote path is mediated: Goni OS -> Goni Council -> OpenRouter (multi-model gateway) -> cloud providers. Goni never calls provider APIs directly; see [blueprint/docs/remote-llm-architecture.md](/blueprint/docs/remote-llm-architecture.md) for data path, budgets, and runtime modes.

- **Workload focus** (human-speed chat and assistant latency, not API-scale throughput):
  - Inference + RAG for 30-40B quantised models  
  - Adapters / LoRA / "personalisation" training only  
  - Heavy full-model fine-tune -> kicked to cloud or GN100-class node
