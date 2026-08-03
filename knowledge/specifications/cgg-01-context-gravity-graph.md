---
id: CGG-01
title: Context Gravity Graph
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: CGG-01 Status: Specified only / roadmap The Context Gravity Graph is Goni''s contract for turning memory from static storage into task-conditioned contextual salience.'
domains:
- specs
aliases:
- CONTEXT-GRAVITY-GRAPH
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: Context Gravity Graph
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# Context Gravity Graph

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Context Gravity Graph
DOC-ID: CGG-01

Status: Specified only / roadmap

The Context Gravity Graph is Goni's contract for turning memory from static
storage into task-conditioned contextual salience. Product language may call
this a "gravitational field": every prior decision, artifact, correction, and
memory can exert pull on future work. The normative contract below uses the
implementation terms: typed nodes, reasoned edges, traversal, salience, decay,
and Context Plane materialization.

This spec extends Governed Memory Retrieval (MEM-RETR-01). It does not change
the `/v1/chat/completions` API, and it does not add a shipping table to the
executable `goni-schema` DSL. Graph databases, ANN indexes, and caches may be
derived backends, but Arrow rows remain the local-first source of truth.

The Context Gravity Graph does not replace retrieval. It governs which
retrieved, remembered, and inferred materials are allowed to exert salience on
a Work Order, then compiles that influence into a `ContextPack` with receipts.
