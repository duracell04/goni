---
id: GONI-SPEC-B5A0BAC88273
title: 2. Pipeline contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'A governed memory pipeline MUST implement these stages: Observe incoming items such as chats, files, notes, tasks, emails, events, corrections, accepted/rejected drafts, and prior outputs.'
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/memory-retrieval.md
  heading: 2. Pipeline contract
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 2. Pipeline contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Pipeline contract

A governed memory pipeline MUST implement these stages:

1. Observe incoming items such as chats, files, notes, tasks, emails, events,
   corrections, accepted/rejected drafts, and prior outputs.
2. Classify each candidate into an explicit memory class:
   `episodic | semantic | procedural | relational | project | policy`.
3. Parse and type source material into structured evidence candidates. Parser
   outputs are part of the security perimeter because parsing errors can create
   incorrect memory and downstream actions.
4. Chunk source material into retrievable units such as decisions, source-backed
   facts, actions, open loops, table regions, and paragraph chunks.
5. Index with dense semantic vectors plus sparse, exact-match, graph, and
   metadata signals where available. Graph retrieval MUST follow CGG-01 when
   edge traversal affects context assembly.
6. Attach explicit metadata for source, timestamp, project, person,
   permissions, quoteability, confidence, validity window, and expiry.
7. Retrieve against the canonical Work Order, not only the raw user utterance.
8. Rerank and filter by task relevance, recency, project fit, source trust,
   permission scope, and policy safety.
9. Verify selected evidence against the Work Order, expected output shape,
   source boundaries, parser confidence, and permission policy.
10. Cite selected evidence with enough source waypoints for audit.
11. Materialize only selected evidence into the Context Plane.
12. Emit receipts for parsing, memory reads, memory writes, and selected
    context materialization when those stages affect output or execution.

For desktop, browser, and vision-derived inputs, stages 1-4 MUST preserve
Desktop Agent Firewall boundary refs. A parser or extractor may produce
candidate chunks without gaining authority to store or reuse them.
