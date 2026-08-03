---
id: MEM-RETR-01
title: Governed Memory Retrieval
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: MEM-RETR-01 Status: Specified only / roadmap Goni memory is governed **Knowledge & Context Engineering**, not a user-managed folder, tag, or "remember this" feature.'
domains:
- memory
- specs
aliases:
- MEMORY-RETRIEVAL
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/memory-retrieval.md
  heading: Governed Memory Retrieval
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# Governed Memory Retrieval

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Governed Memory Retrieval
DOC-ID: MEM-RETR-01

Status: Specified only / roadmap

Goni memory is governed **Knowledge & Context Engineering**, not a
user-managed folder, tag, or "remember this" feature. The user states work
intent; the kernel classifies, parses, indexes, filters, retrieves, reranks,
verifies, cites, and receipts memory as system work.

Observation and context extraction do not create memory authority. Screen
frames, OCR, accessibility trees, summaries, embeddings, audio transcripts, and
layout facts may enter durable memory only through a memory grant mediated by
BOUND-01.

The term RAG may still appear in compatibility notes, UI labels, and supplier
comparisons. In Goni's architecture, the stronger layer name is Knowledge &
Context Engineering because retrieval is only one stage in an audit-capable
context pipeline.
