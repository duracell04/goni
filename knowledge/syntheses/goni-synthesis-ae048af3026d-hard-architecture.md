---
id: GONI-SYNTHESIS-AE048AF3026D
title: Hard (architecture)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Memory Plane API:** store(entry), recall(query, mode), forget(selector), summarize(range), audit(entry_id); engines/controllers use this interface only.'
domains:
- memory
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/memory-architecture.md
  heading: Hard (architecture)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# Hard (architecture)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Hard (architecture)

- **Memory Plane API:** `store(entry)`, `recall(query, mode)`, `forget(selector)`, `summarize(range)`, `audit(entry_id)`; engines/controllers use this interface only.
- **Types & lifecycle:** `working` (session), `episodic` (raw -> distilled -> archived/tombstoned), `semantic` (facts with decay/pin), `procedural` (versioned skills). States carry `importance` that decays unless reinforced.
- **Local-first:** Long-term memory stays on-device (Arrow + vector/graph backend). Council/cloud paths see at most distilled facts or session context when explicitly allowed.
- **Backend-pluggable:** Default is Arrow + Qdrant/Lance; backend can be swapped (e.g. OpenMemory/Mem0/curved index) if it preserves the API and lifecycle semantics.
- **Traceability & redaction:** Recall returns waypoints (`why this memory`); `forget` performs graph-aware redaction and reindexing.
