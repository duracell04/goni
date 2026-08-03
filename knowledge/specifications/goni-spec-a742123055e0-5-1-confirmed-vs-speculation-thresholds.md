---
id: GONI-SPEC-A742123055E0
title: 5.1 Confirmed vs speculation thresholds
type: specification
status: draft
implementation_state: specified_only
proposition: 'For MemoryEntries: A claim is **confirmed** if confirmed_by_event_id is present, or if source_chunk_ids is non-empty, confidence meets the policy threshold, and conflict_state is not contradictory.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 5.1 Confirmed vs speculation thresholds
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 5.1 Confirmed vs speculation thresholds

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5.1 Confirmed vs speculation thresholds

For MemoryEntries:
- A claim is **confirmed** if `confirmed_by_event_id` is present, or if
  `source_chunk_ids` is non-empty, `confidence` meets the policy threshold, and
  `conflict_state` is not contradictory.
- Otherwise, the claim MUST be stored as `hypothesis` or `derived` with a
  `ttl_ms` or `review_at` value, and MUST NOT be promoted to `fact` without
  new evidence.
