---
id: GONI-IMAP-C7EA077A890E
title: Relationship to RAG (retrieval is a tool, not cognition)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'RAG is represented as tool calls and Memory Plane queries: encoders produce embeddings for the query and current state, predictor decides whether to retrieve, retrieved items are merged into latent state as evidence.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Relationship to RAG (retrieval is a tool, not cognition)
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Relationship to RAG (retrieval is a tool, not cognition)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Relationship to RAG (retrieval is a tool, not cognition)

RAG is represented as tool calls and Memory Plane queries:
- encoders produce embeddings for the query and current state,
- predictor decides whether to retrieve,
- retrieved items are merged into latent state as evidence.

Key point: retrieval augments state; it does not replace the predictor. The predictor decides:
- what to retrieve (scope),
- how to weight evidence,
- when to stop retrieving.
