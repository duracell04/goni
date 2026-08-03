---
id: GONI-IMAP-A610DC8C0731
title: 1. Role in the system
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The **Vector Database** (VecDB) is the retrieval backend for the Context Plane (??): Stores embeddings and metadata for chunks, Performs approximate nearest neighbour (ANN) search, Returns candidates as Arrow RecordBatches on the Arrow Spine (??).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 1. Role in the system
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 1. Role in the system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role in the system

The **Vector Database** (VecDB) is the retrieval backend for the Context Plane (??):

- Stores embeddings and metadata for chunks,
- Performs approximate nearest neighbour (ANN) search,
- Returns candidates as Arrow RecordBatches on the Arrow Spine (??).

It is a concrete implementation of the retrieval part feeding the submodular context selector.

Note: retrieval is treated as a Memory Plane capability invoked by the
predictor as evidence. It augments latent state; it is not the cognitive core.
See `blueprint/30-specs/latent-state-contract.md` and
`blueprint/30-specs/tool-capability-api.md`.

---
