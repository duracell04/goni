---
id: GONI-SPEC-09BE1D581906
title: 4. Edges
type: specification
status: draft
implementation_state: specified_only
proposition: KnowledgeGraphEdges is a specified-only future table concept.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 4. Edges
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 4. Edges

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Edges

`KnowledgeGraphEdges` is a specified-only future table concept. It is not part
of the shipping schema DSL until a schema revision promotes it.

An implementation that materializes graph edges MUST preserve these logical
fields, whether stored as Arrow rows, derived indexes, or replayable artifacts:

| Field | Meaning |
| --- | --- |
| `edge_id` | UUIDv7 row ID if persisted as a canonical row. |
| `source_ref` | Typed source node reference, e.g. `MemoryEntries:<uuid>`. |
| `target_ref` | Typed target node reference. |
| `edge_type` | Controlled label from the ontology below. |
| `explicit_user_weight` | Optional user-set weight in `[0, 1]`. |
| `system_inferred_weight` | Optional parser/model/system-inferred weight in `[0, 1]`. |
| `usage_reinforced_weight` | Optional reinforcement weight from repeated accepted use in `[0, 1]`. |
| `final_weight` | Derived inspectable weight before decay and policy filtering. |
| `reason_summary` | Bounded summary of why the edge exists. |
| `reason_ref` | Optional hash or source ref for replaying the rationale. |
| `scope_refs` | Project, person, Work Order, policy, or task refs where the edge applies. |
| `confidence` | Float in `[0, 1]` expressing extraction or assertion confidence. |
| `permission_scope` | Finite permission label compatible with MemoryEntries. |
| `quoteability` | Finite quoteability label when edge traversal may surface source content. |
| `valid_from`, `valid_until` | Temporal validity window. |
| `ttl_ms` | Optional expiry budget. |
| `decay_policy` | Finite label or config ref for temporal decay. |
| `conflict_state` | Finite state for normal, conflicted, superseded, quarantined, or pending-review edges. |
| `provenance` | Parser, model, user action, receipt, policy, and source refs. |

Raw edge rationale text MUST NOT be stored in edge rows by default. Long text
belongs only in permitted Knowledge or Context Plane fields, currently
`Chunks.text` and `Prompts.text`. Edge rows use summaries, hashes, and refs.
