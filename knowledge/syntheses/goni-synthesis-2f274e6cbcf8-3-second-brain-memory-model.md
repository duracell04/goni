---
id: GONI-SYNTHESIS-2F274E6CBCF8
title: 3. Second-brain memory model
type: synthesis
status: draft
implementation_state: specified_only
proposition: The memory labels below are a reader-facing view of existing storage and lifecycle contracts.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/60-cognitive-exocortex-model.md
  heading: 3. Second-brain memory model
  revision: cdf162b26a4fe7d78e6daa6039696e89ee0ef17f
---

# 3. Second-brain memory model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Second-brain memory model

The memory labels below are a reader-facing view of existing storage and
lifecycle contracts.

| Memory view | Canonical placement and lifecycle |
| --- | --- |
| **Evidence** | Source material belongs in governed Knowledge-plane records and content-addressed artifacts, with source and integrity metadata. Evidence may be append-only or integrity-protected where policy requires, but remains subject to authorized retention, deletion, redaction, and tombstoning. |
| **Episodic** | Chronological events and interactions use the `episodic` MemoryEntry class and retain source and receipt references. |
| **Semantic** | Facts, claims, decisions, and derived understanding use governed MemoryEntries with confidence, validity, conflict, permission, and provenance metadata. The existing `relational`, `project`, and `policy` classes remain first-class and are not collapsed into this label. |
| **Procedural** | Reusable methods, preferences about how work is done, and governed skills use the `procedural` or `policy` classes according to authority and scope. |
| **Working** | Task-scoped context lives in the Context Plane and hot latent state. It expires or is discarded unless a separate, policy-mediated memory grant authorizes consolidation. |

The canonical fields and finite memory classes remain those in the
[MVP schemas](/blueprint/software/50-data/51-schemas-mvp.md) and
[memory retrieval contract](/blueprint/30-specs/memory-retrieval.md).
Observation, extraction, or appearance in working context never grants durable
memory authority by itself.
