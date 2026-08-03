---
id: GONI-IMAP-A505EC9528A5
title: Embeddings
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: embedding_id = row_id Fields: chunk_id: fixed_size_binary[16], model_id: dict<uint8, utf8>, vector: fixed_size_list<float32>[1536], dim: uint16 Notes: Lance index on vector.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: Embeddings
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Embeddings

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Embeddings
- PK: `embedding_id = row_id`
- Fields: `chunk_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `vector: fixed_size_list<float32>[1536]`, `dim: uint16`
- Notes: Lance index on `vector`.
