---
id: GONI-SPEC-8BAE4C018BE1
title: 5.1 Conflict-preserving query modes
type: specification
status: draft
implementation_state: specified_only
proposition: Contradiction does not grant the graph authority to manufacture consensus.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 5.1 Conflict-preserving query modes
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 5.1 Conflict-preserving query modes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 Conflict-preserving query modes

Contradiction does not grant the graph authority to manufacture consensus. A
Work Order that encounters materially conflicting claims MUST preserve both
sides and select behavior according to the query purpose:

| Query mode | Required behavior |
| --- | --- |
| `descriptive` | Return the material competing claims, provenance, confidence, validity, and conflict basis. |
| `historical` | Apply the requested validity window and distinguish later amendments or supersession. |
| `operational` | Apply a controlling rule only when the principal or an explicitly delegated role supplied one; retain material dissent as context. |

If an operational query has no authorized controlling rule, the runtime MUST
surface the conflict, ask, or escalate under the Work Order rather than infer
authority from graph weight, model confidence, source count, or majority
agreement.

A conflict resolution may change which node controls future operational
selection, but MUST NOT delete or rewrite the competing node, its provenance,
or the `contradicts` relationship. The decision and its authority basis MUST be
receipt-linked. Formal policy and observed practice remain separate nodes or
claims even when one controls the current operation.
