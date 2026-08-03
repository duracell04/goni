---
id: GONI-SYNTHESIS-84822AD77041
title: E) Memory, RAG, state, and knowledge layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: | Project | Confidence | Goni relevance | | mem0 | verified | Agent memory layer candidate.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: E) Memory, RAG, state, and knowledge layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# E) Memory, RAG, state, and knowledge layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### E) Memory, RAG, state, and knowledge layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| mem0 | `verified` | Agent memory layer candidate. |
| MemX | `verified` | Local-first Rust/libSQL memory service with hybrid retrieval and low-confidence rejection; backend experiment, not a full agent. |
| LightMem | `verified` | Research-backed memory framework with local-model support and deferred consolidation patterns. |
| EverOS | `verified` | Local-first memory ecosystem with Markdown/SQLite/index methods, integrations, and evaluation assets. |
| Zep | `verified` | Long-term conversational memory/state service. |
| Chroma / ChromaDB | `verified` | Local vector store candidate. |
| Qdrant | `verified` | Current prototype-aligned vector DB candidate. |
| Milvus | `verified` | Scalable vector DB candidate. |
| Weaviate | `verified` | Vector DB and hybrid retrieval candidate. |
| pgvector | `verified` | Postgres vector extension path. |
| Redis | `verified` | Cache/state/checkpoint substrate. |
| Neon | `verified` | Serverless Postgres supplier candidate; more cloud than local-first. |
| LlamaIndex | `verified` | RAG and agentic retrieval framework. |
| RAGFlow | `verified` | Document understanding and RAG application framework. |
| Unstructured | `verified` | Document parsing and ingestion pipeline. |
| Docling | `verified` | Document conversion/parsing pipeline. |
| Marker | `verified` | PDF/document-to-Markdown extraction candidate. |
| Neo4j | `verified` | Graph database for knowledge graph / GraphRAG experiments. |
| Kuzu | `verified` | Embedded graph database candidate. |
| Marqo | `verified` | Vector search platform candidate. |
| LanceDB | `verified` | Embedded/vector data lake candidate. |
| Obsidian AI/plugin patterns | `needs verification` | Personal knowledge integration pattern, not one stable upstream component. |

Goni implication:

- VecDB is only one piece of memory.
- Durable state, checkpoints, selective forgetting, graph memory, and provenance
  must remain under Goni governance even when external stores are used.
