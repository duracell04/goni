---
id: CONV-INGEST-01
title: Governed Conversation Ingestion
type: specification
status: draft
implementation_state: specified_only
proposition: >-
  A conversation is an observation stream and provenance source rather than
  GONI's canonical unit of knowledge organization; useful content should be
  decomposed into governed, retrievable representations linked to Work Orders,
  projects, entities, decisions, and memory objects.
domains:
- memory
- context
- retrieval
aliases:
- CONVERSATION-AS-OBSERVATION-STREAM
relations:
- type: refines
  target: MEM-RETR-01
- type: depends_on
  target: CGG-01
- type: depends_on
  target: CTX-STATE-01
- type: depends_on
  target: TASK-CHECKPOINT-01
sources: []
artifacts: []
uncertainty: Extraction quality, retention policy, and retrieval weighting require empirical calibration and must remain principal-governed.
legacy: []
---

# Governed Conversation Ingestion

> Status boundary: this is a new specified-only contract. It defines intended decomposition and authority boundaries, not a completed ingestion pipeline.

A chat thread is useful as an interaction surface and source record, but it should not become the primary organizational structure for long-term personal knowledge. GONI SHOULD treat conversation as an event stream that can feed multiple governed representations.

A conversation ingestion pipeline may derive:

```text
conversation
  -> retained source transcript or source refs
  -> chunks
  -> lexical / sparse index entries
  -> embeddings
  -> entities and graph links
  -> temporal and project metadata
  -> candidate facts
  -> candidate preferences
  -> decisions
  -> tasks and commitments
  -> claims and evidence links
  -> episode or task checkpoints
```

Model extraction produces candidates, not authority. A statement inferred from conversation MUST NOT become a principal assertion, controlling rule, or durable preference merely because a model extracted it. Promotion into canonical memory remains governed by memory class, provenance, confidence, authorization, correction state, and the Memory Write Gate.

Retrieval over historical conversations SHOULD be hybrid rather than embedding-only. Candidate ranking may combine:

[
R = f(
\text{semantic similarity},
\text{lexical match},
\text{entity/graph relevance},
\text{time},
\text{project scope},
\text{importance},
\text{decision status},
\text{source authority}
).
]

The organizational primitive is therefore the governed object or Work Order, not the transcript container. One conversation may update several projects or entities; one project may retrieve evidence from many conversations. Raw transcripts remain provenance sources subject to retention policy, while active model context receives only the bounded material selected for the current task.
