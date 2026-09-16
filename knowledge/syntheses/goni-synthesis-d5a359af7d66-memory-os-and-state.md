---
id: GONI-SYNTHESIS-D5A359AF7D66
title: Memory OS and state
type: synthesis
status: draft
implementation_state: specified_only
proposition: '| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence | | MemGPT / Letta | https://arxiv.org/abs/2310.08560 | Tabulation, memoization, rolling state, virtual context | Treat context as managed memory with explicit movement between short and long-term stores.'
domains:
- research
aliases: []
relations: []
sources:
- SRC-PACKER2023-MEMGPT
- SRC-KWON2023-PAGEDATTENTION
- SRC-JUSTVUGG2026-COLIBRI
- SRC-GOOGLE2026-LITERT-LM
artifacts: []
uncertainty: The original rows are preserved from the migrated draft. New rows distinguish academic, repository, and vendor engineering evidence and do not promote GONI implementation status.
legacy:
- path: blueprint/docs/references/gonios-research-neighbor-map.md
  heading: Memory OS and state
  revision: 08e1061f9ab1e1a95e22a924fdc9970e0585851b
---

# Memory OS and state

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Memory OS and state

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| MemGPT / Letta | https://arxiv.org/abs/2310.08560 | Tabulation, memoization, rolling state, virtual context | Treat context as managed memory with explicit movement between short and long-term stores. | Do not equate chat history management with Goni's full authority, receipt, and policy system. | `primary-source verified` |
| Letta | https://github.com/letta-ai/letta | Agent memory runtime | Study practical APIs for long-running agents with external memory. | Do not let an agent framework own canonical Goni memory or policy state. | `primary-source verified` |
| MemOS | https://arxiv.org/abs/2505.22101 | Memory as OS-level resource | Study memory lifecycle, memory scheduling, and memory service boundaries for LLM systems. | Do not claim Goni implements a memory OS unless the memory object contract and runtime wiring exist. | `primary-source verified` |
| MemPalace | https://github.com/mempalace/mempalace | Hierarchical local memory and retrieval | Compare memory topology, hierarchy, and retrieval organization against Vault and Context Gravity Graph designs. | Do not import benchmark claims without reproducing them in Goni Lab. | `primary-source verified` |
| Basic Memory | https://github.com/basicmachines-co/basic-memory | Local-first writable memory, graph-like notes | Study Markdown-backed local memory and MCP-facing workflows as a simple Vault substrate pattern. | Do not treat Markdown notes as sufficient for receipts, expiry, invalidation, or policy mediation. | `primary-source verified` |
| PagedAttention / vLLM | https://arxiv.org/abs/2309.06180 | Physical inference-memory residency | Use paging as a systems precedent for decoupling logical availability from contiguous physical residency. | Do not treat KV-cache paging as direct evidence for semantic memory, provenance, or authority. | `academic primary source` |
| Colibrì | https://github.com/JustVugg/colibri/blob/main/README.md | Heterogeneous model-memory placement | Study heat-based placement, caching, migration, and prefetch as engineering patterns for residency management. | Do not treat repository-level performance claims as independently validated or equate expert placement with cognitive memory. | `repository primary source` |
| LiteRT-LM | https://developers.googleblog.com/blazing-fast-on-device-genai-with-litert-lm/ | Stateful local inference and recoverable KV state | Study session save/restore, serialized KV state, memory locality, and dynamic component loading as local continuity mechanisms. | Do not equate persisted inference state with evidence-bearing memory or infer GONI authority semantics from vendor runtime features. | `vendor engineering source` |

The updated comparison motivates a distinction that the original research-neighbor map did not make explicit: model-memory residency, cognitive-memory residency, and delegated authority are separate architectural dimensions. The dedicated synthesis `GONI-SYNTHESIS-83E91B709969` develops that distinction without changing the evidentiary status of the projects listed here.
