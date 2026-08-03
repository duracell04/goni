---
id: GONI-IMAP-10A9F822C8AC
title: 3.2 Result schema (Arrow)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Search results MUST at least expose: chunk_id: Utf8, similarity: Float32, oken_count: UInt32, source_meta: LargeBinary or similar.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 3.2 Result schema (Arrow)
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 3.2 Result schema (Arrow)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Result schema (Arrow)

Search results MUST at least expose:

* chunk_id: Utf8,
* similarity: Float32,
* 	oken_count: UInt32,
* source_meta: LargeBinary or similar.

This schema is what goni-context expects for the submodular objective and budget accounting.

Adjacent memory and retrieval candidates include Chroma/ChromaDB, Qdrant,
Milvus, Weaviate, pgvector, Redis, Neon, LlamaIndex, RAGFlow, Unstructured,
Docling, Marker, Neo4j, Kuzu, Marqo, LanceDB, mem0, Zep, and Obsidian
AI/plugin patterns. They are tracked in
[Adjacent Projects](/blueprint/docs/adjacent-projects.md). Goni treats these as
storage, parsing, retrieval, or state backends; memory governance, selective
forgetting, and receipt provenance stay in the Goni control plane.

---
