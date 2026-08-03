---
id: GONI-THESIS-B09E3475FE2C
title: '5. Memory: The Continuity Layer'
type: thesis
status: draft
implementation_state: specified_only
proposition: A personal AI system requires durable, structured, and accountable memory.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: '5. Memory: The Continuity Layer'
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 5. Memory: The Continuity Layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Memory: The Continuity Layer

A personal AI system requires durable, structured, and accountable memory.
Without memory discipline, personalization collapses into a mixture of chat
transcripts, vector embeddings, prompt fragments, and ungoverned local state.
Goni addresses this through the Arrow Spine: a typed, auditable memory substrate
that gives the system a canonical representation of documents, chunks,
embeddings, prompts, context items, tasks, audit records, capability tokens,
redaction events, state snapshots, memory entries, model calls, platform
signals, and metrics.

The memory plane is governed by three important design principles. First,
persistent and transient entities are intended to be represented in canonical
tables rather than scattered across ad hoc stores. Second, cross-component APIs
prefer structured batches and opaque identifiers rather than uncontrolled copies
of raw data. Third, long-form raw text is confined to explicitly permitted
text-bearing tables. The current schema MVP and its status are described in
[software/50-data/51-schemas-mvp.md](/blueprint/software/50-data/51-schemas-mvp.md).

The academic importance of this design is that Goni treats memory not as a
convenience feature but as an institutional primitive. Memory determines what
the system knows, what it can cite, what it can retrieve, what it can forget,
and what evidence can be used to justify action. In a delegated AI system,
memory is not merely context. It is part of the authority structure.

This is why emerging memory concepts such as graph-influenced retrieval and
ContextPacks remain subordinate to memory governance. The
[Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md) is a
specified-only design for task-conditioned salience, not a second source of
truth for raw text, permissions, or retention state.
