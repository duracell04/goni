---
id: GONI-SPEC-B49DB23CF412
title: 3. ContextPack
type: specification
status: draft
implementation_state: specified_only
proposition: A ContextPack is the compiled context bundle produced by graph traversal, reranking, compression, and policy filtering for one Work Order.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 3. ContextPack
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 3. ContextPack

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. ContextPack

A `ContextPack` is the compiled context bundle produced by graph traversal,
reranking, compression, and policy filtering for one Work Order.

`ContextPack` is specified only in CGG-01. It is not a shipping canonical table
or API object until a later schema/API revision promotes it. Implementations may
represent it as a replayable artifact, receipt-linked metadata, or derived
Context Plane state, but they MUST preserve this logical shape:

```yaml
context_pack_id:
work_order_id:
graph_snapshot_id:
scoring_policy_id:
decay_policy_id:
permission_filter_ref:
token_budget:
selected_context_items:
excluded_candidates:
compression_policy:
assembly_reason:
receipt_ref:
created_at:
provenance:
```

`selected_context_items` references the material selected for the prompt bundle,
usually `ContextItems` plus source waypoints. `excluded_candidates` records
bounded refs and omission reasons for high-salience or high-similarity
candidates that did not enter the pack. `assembly_reason` is a bounded summary
or hash/ref pair, not raw free-form rationale text.
