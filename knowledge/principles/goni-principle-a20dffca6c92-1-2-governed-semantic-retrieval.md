---
id: GONI-PRINCIPLE-A20DFFCA6C92
title: 1.2 Governed semantic retrieval
type: principle
status: draft
implementation_state: specified_only
proposition: Memory retrieval is a composite decision function, not pure vector similarity.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/10-axioms-and-planes.md
  heading: 1.2 Governed semantic retrieval
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 1.2 Governed semantic retrieval

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2 Governed semantic retrieval

Memory retrieval is a composite decision function, not pure vector similarity.

The Memory Plane SHOULD combine dense embeddings, sparse retrieval, metadata
filters, recency weighting, permission checks, and reranking. Dense embeddings
provide semantic candidate generation; symbolic metadata and policy constraints
make retrieval accountable.

Embedding vectors MUST NOT be treated as interpretable feature fields.
Attributes that require inspection or enforcement, such as person, project,
source, timestamp, permission, validity, quoteability, or retention class, MUST
be stored explicitly as metadata.

Chunking is part of the memory contract: the retrievable unit must be chosen so
that context remains meaningful, citeable, permission-aware, and correctable.
Poor chunking can destroy retrieval quality even when the embedding model is
strong.

The retrieval target is higher discriminability between task-relevant and
task-irrelevant memories under a query-specific, policy-bounded retrieval
function. It is not “more dimensions” as such.
