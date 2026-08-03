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
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
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
