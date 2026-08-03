---
id: GONI-PRINCIPLE-D0161AB0CE1A
title: 1. Planes (partitioning)
type: principle
status: draft
implementation_state: specified_only
proposition: '| Plane | Symbol | Lifetime | Sensitivity | Storage | Primary Keys | | Knowledge | 𝒜 | Permanent | High (raw text) | Parquet + Lance v2 | doc_id, chunk_id, embedding_id | | Context | 𝒳 | ≤ 24 h | High (live prompts) | Memory + optional spill to encrypted tmp | request_id, context_id | | Control | 𝒦 | Permanent | Low (metadata only) | WAL + Parquet | request_id, task_id |'
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
  heading: 1. Planes (partitioning)
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 1. Planes (partitioning)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Planes (partitioning)

| Plane | Symbol | Lifetime | Sensitivity | Storage | Primary Keys |
|-------|--------|----------|-------------|---------|--------------|
| Knowledge | 𝒜 | Permanent | High (raw text) | Parquet + Lance v2 | `doc_id`, `chunk_id`, `embedding_id` |
| Context | 𝒳 | ≤ 24 h | High (live prompts) | Memory + optional spill to encrypted tmp | `request_id`, `context_id` |
| Control | 𝒦 | Permanent | Low (metadata only) | WAL + Parquet | `request_id`, `task_id` |
| Execution | ℰ | Permanent | Low (aggregates) | Parquet (append-only) | `span_id`, `call_id` |
