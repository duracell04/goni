---
id: GONI-SPEC-2F762878317A
title: 8. Context Assembly Flow
type: specification
status: draft
implementation_state: specified_only
proposition: 'Graph-influenced context assembly MUST preserve the MEM-RETR-01 pipeline and add graph traversal as a bounded retrieval stage: Bind retrieval to a WorkOrder; if no Work Order exists, create one or record why a read-only lookup is allowed without it.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 8. Context Assembly Flow
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 8. Context Assembly Flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Context Assembly Flow

Graph-influenced context assembly MUST preserve the MEM-RETR-01 pipeline and
add graph traversal as a bounded retrieval stage:

1. Bind retrieval to a `WorkOrder`; if no Work Order exists, create one or
   record why a read-only lookup is allowed without it.
2. Generate seed candidates from dense, sparse, exact-match, metadata, and
   policy-bounded lookups.
3. Expand from seeds through permitted graph edges under configured depth,
   fanout, token, and latency budgets.
4. Score candidates with the salience function.
5. Filter by permission, quoteability, validity, conflict state, source trust,
   and parser confidence.
6. Rerank candidates and run existing submodular selection under token budget.
7. Choose compression forms for selected candidates.
8. Compile a `ContextPack` for the Work Order.
9. Materialize only selected evidence into the Context Plane as `ContextItems`
   or prompt material derived from selected refs.
10. Cite selected evidence with source waypoints sufficient for audit.
11. Emit receipts for memory reads, graph retrieval basis, omissions,
    compression choices, and context materialization when retrieval affects
    output or execution.

Graph traversal is not a bypass around policy. It is one retrieval signal among
dense, sparse, exact-match, metadata, and reranking signals.
