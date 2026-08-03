---
id: GONI-IMAP-CB3F1517E50C
title: 2.1 Responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Storage & indexing** Maintain an index over (chunk_id, embedding, token_count, source_metadata).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 2.1 Responsibilities
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 2.1 Responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Responsibilities

- **Storage & indexing**
  - Maintain an index over (chunk_id, embedding, token_count, source_metadata).

- **ANN search**
  - Given a query embedding q, return top-K nearest neighbours with similarity scores.

- **Arrow integration**
  - Expose results as RecordBatch with a fixed schema usable by ??.
