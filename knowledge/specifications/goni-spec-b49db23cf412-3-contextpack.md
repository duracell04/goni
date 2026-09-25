---
id: GONI-SPEC-B49DB23CF412
title: 3. ContextPack
type: specification
status: draft
implementation_state: specified_only
proposition: A ContextPack is the model-independent compiled semantic context bundle produced for one Work Order from governed evidence, memory, skills, compression policy, visibility constraints, and resource budgets.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft and refined by CTX-COMP-01; no shipping table or API object is claimed.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 3. ContextPack
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 3. ContextPack

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. ContextPack

A `ContextPack` is the model-independent compiled semantic context bundle
produced for one Work Order. `CTX-COMP-01` governs its construction.

`ContextPack` remains specified only. It is not a shipping canonical table or
API object until a later schema/API revision promotes it. Implementations may
represent it as a replayable artifact, receipt-linked metadata, or derived
Context Plane state, but they MUST preserve this logical shape:

```yaml
context_pack_id:
work_order_id:
compiler_version:
compiler_policy_hash:
source_snapshot_refs:
graph_snapshot_id:
scoring_policy_id:
decay_policy_id:
permission_filter_ref:
skill_fragment_refs:
token_budget:
selected_context_items:
excluded_candidates:
compression_policy:
assembly_reason:
context_pack_hash:
receipt_ref:
created_at:
provenance:
```

`selected_context_items` references material selected for semantic
materialization, usually `ContextItems` plus source waypoints.
`skill_fragment_refs` records the explicit procedural fragments selected under
SKILL-REG-01. `excluded_candidates` records bounded refs and omission reasons
for candidates crossing the configured salience, similarity, or audit
threshold.

`assembly_reason` is a bounded summary or hash/ref pair, not raw free-form
rationale text.

A ContextPack deliberately excludes model-specific tokenization, chat-template
encoding, special-token layout, and runtime KV state. Those belong to the
runtime-serialization layer rather than this semantic contract.
