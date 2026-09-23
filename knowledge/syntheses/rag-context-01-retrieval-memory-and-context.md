---
id: RAG-CONTEXT-01
title: Retrieval, memory, and context are distinct system functions
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni treats RAG as one information-retrieval mechanism inside context engineering and distinguishes retrieval, persistent memory, task state, belief state, and the model context compiled for each inference.
domains:
- system
- research
- context
- memory
aliases:
- RAG and context engineering
relations:
- type: refines
  target: GONI-THESIS-B09E3475FE2C
sources:
- SRC-LEWIS2020-RAG
- SRC-MEI2025-CONTEXT-ENGINEERING
- SRC-HU2026-AGENT-MEMORY-SURVEY
artifacts: []
uncertainty: Context-engineering and agent-memory terminology is still evolving; Goni uses these distinctions to constrain system responsibilities rather than to claim a universal taxonomy.
legacy: []
---

# Retrieval, memory, and context are distinct system functions

RAG is not a stage between an LLM and an agent. It is one way to acquire information for inference.

Goni's context path is conceptually:

```text
objective + task state + belief state + relevant memory + retrieved evidence
  -> selection / reranking / compression / normalization
  -> bounded model context
  -> model inference
```

## Distinctions

**Retrieval / RAG** selects information from external or persistent stores for a current inference.

**Persistent memory** retains governed information across tasks and sessions and therefore needs formation, provenance, revision, conflict handling, expiry or forgetting, and retrieval policies.

**Task state** records what the current workflow has done and what remains.

**Belief state** represents the agent's current epistemic position, including uncertainty and contradictions.

**Model context** is the finite inference-time payload assembled from the preceding objects plus instructions, policies, tool schemas, and recent observations.

## Context engineering

Goni uses context engineering as the broader systems abstraction. The context engine may:

- retrieve or generate candidate information,
- filter, rerank, compress, normalize, and cite it,
- allocate a context budget,
- preserve instruction and provenance boundaries,
- invalidate stale or superseded material,
- select relevant memories and task state,
- construct the smallest sufficient context for the requested reasoning step.

The goal is to replace conversational archaeology with an explicit compiled context whose contents and provenance can be inspected and reproduced.
