---
id: GONI-PROPOSAL-C07C45BF97AF
title: Memory components
type: proposal
status: draft
implementation_state: specified_only
proposition: '| Project | Maturity and local path | Memory behavior | External dependencies / telemetry | Relevance to Goni | | MemX | Local-first Rust memory service centered on a single libSQL file | Hybrid retrieval, importance/confidence handling, and low-confidence rejection | Embedding endpoint requirements must be checked for the selected deployment; telemetry status was not established from the reviewed documentation | Candidate embedded-memory experiment, not an end-to-end secretary or authority laye'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Memory components
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Memory components

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Memory components

| Project | Maturity and local path | Memory behavior | External dependencies / telemetry | Relevance to Goni |
| --- | --- | --- | --- | --- |
| [MemX](https://memx.me/) | Local-first Rust memory service centered on a single libSQL file | Hybrid retrieval, importance/confidence handling, and low-confidence rejection | Embedding endpoint requirements must be checked for the selected deployment; telemetry status was not established from the reviewed documentation | Candidate embedded-memory experiment, not an end-to-end secretary or authority layer |
| [LightMem](https://github.com/zjunlp/LightMem) | Research-backed memory framework with Ollama, vLLM, and hosted-provider support | Separates lightweight online work from deferred ("offline") consolidation in the processing sense and supports storage/retrieval/update experiments | Network locality follows the configured model and embedding providers; telemetry status was not established from the reviewed repository | Evaluation and consolidation reference; adoption requires mapping outputs to Goni provenance and lifecycle contracts |
| [EverOS](https://github.com/EverMind-AI/EverOS) | Active local-first memory ecosystem with an offline educational demo and broader flows that may use external model APIs | Markdown-, SQLite-, and index-backed memory methods, recall, self-evolution experiments, and memory benchmarks | Broader flows may depend on external model APIs; telemetry status was not established from the reviewed repository | Useful backend and benchmark candidate; cannot own Goni memory authority or replace receipt semantics |

Projects without an authoritative URL establishing identity, license,
architecture, and current status are excluded. Volatile GitHub star counts are
also omitted because they do not establish technical maturity.
