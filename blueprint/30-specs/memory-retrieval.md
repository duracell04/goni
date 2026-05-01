---
id: MEM-RETR-01
type: SPEC
status: specified_only
---
# Governed Memory Retrieval
DOC-ID: MEM-RETR-01

Status: Specified only / roadmap

Goni memory is governed **Knowledge & Context Engineering**, not a
user-managed folder, tag, or "remember this" feature. The user states work
intent; the kernel classifies, parses, indexes, filters, retrieves, reranks,
verifies, cites, and receipts memory as system work.

The term RAG may still appear in compatibility notes, UI labels, and supplier
comparisons. In Goni's architecture, the stronger layer name is Knowledge &
Context Engineering because retrieval is only one stage in an audit-capable
context pipeline.

## 1. Scope

This spec applies to durable personal memory, project memory, policy memory,
and retrieved evidence used to assemble Context Plane material.

It does not define a concrete vector database, embedding model, or graph
backend. Backends are swappable if they preserve the contract below.

## 2. Pipeline contract

A governed memory pipeline MUST implement these stages:

1. Observe incoming items such as chats, files, notes, tasks, emails, events,
   corrections, and prior outputs.
2. Classify each candidate into an explicit memory class:
   `episodic | semantic | procedural | relational | project | policy`.
3. Parse and type source material into structured evidence candidates. Parser
   outputs are part of the security perimeter because parsing errors can create
   incorrect memory and downstream actions.
4. Chunk source material into retrievable units such as decisions, source-backed
   facts, actions, open loops, table regions, and paragraph chunks.
5. Index with dense semantic vectors plus sparse, exact-match, graph, and
   metadata signals where available.
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

## 3. Work Order binding

Memory retrieval MUST be bound to `work_order_id` when retrieval is performed
for delegated or tool-mediated work. Retrieval inputs SHOULD include:

- goal summary,
- done contract,
- task class,
- project/person constraints,
- policy hash,
- risk class,
- output shape.

If no Work Order exists, retrieval MUST either create one or record why a
read-only lookup was allowed without one.

## 4. Receipt contract

Every mediated action that reads memory MUST include `memory_read_refs` in its
receipt. Use an empty list when no memory was read.

Every mediated action that changes memory MUST continue to include
`memory_diff_refs`. Use an empty list when no memory mutation occurred.

When retrieval affects output or execution, receipts SHOULD include
`retrieval_basis` with:

- retrieval mode (`dense | sparse | hybrid | graph | mixed`),
- index refs or versions,
- reranker id or policy,
- selected context refs,
- source trust and permission filters,
- policy hash.

Receipt fields MUST NOT store raw source text by default.

When parsing affects memory, context, or execution, receipts SHOULD include
`parser_basis` with:

- source hash and source URI/ref,
- parser ID and parser version,
- parsed structure kind (`text | table | form | email | calendar | code |
  mixed`),
- chunk boundary refs,
- confidence flags and extraction warnings,
- produced chunk IDs or memory IDs,
- policy hash and permission filters.

Parser receipts MUST NOT store raw extracted text by default. They store hashes,
refs, structure summaries, and confidence metadata sufficient to replay or
challenge the parse.

## 5. Evidence anchors

The architecture is supported by prior work on retrieval-augmented generation,
sentence embeddings, dense retrieval, local-first software, proactive dialogue,
and memory-agent evaluation. These sources support the design hypothesis; they
do not prove that Goni is better before product evaluation.
[[lewis2020-rag]] [[reimers2019-sbert]] [[karpukhin2020-dpr]]
[[kleppmann2019-local-first]] [[deng2023-proactive-dialogue]]
[[hu2025-memoryagentbench]]

## 6. Safety invariants

- Untrusted text MUST NOT become control-plane instruction without policy
  mediation.
- Parser output MUST NOT become durable memory or action context without source
  refs, parser identity, confidence metadata, and policy filtering.
- Stale, expired, or conflicted memory MUST be filtered, demoted, or surfaced as
  uncertainty.
- Private memory MUST NOT be sent to remote runtimes unless policy explicitly
  allows the destination and purpose.
- Deletion or redaction MUST trigger reindexing or tombstoning sufficient to
  prevent normal retrieval.
- Memory answers that rely on retrieved evidence SHOULD expose source refs or
  waypoints sufficient for audit.

## 7. Upstream

- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 8. Downstream

- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Retrieval component](/blueprint/software/retrieval/README.md)
- [Vector database](/blueprint/software/30-components/vecdb.md)

## Conformance tests

- A retrieval-mediated action emits `memory_read_refs`.
- A memory mutation emits `memory_diff_refs`.
- A parser-mediated memory write emits `parser_basis`.
- Retrieval against the same Work Order, fixed index, fixed reranker, and fixed
  policy hash is deterministic.
- Expired or policy-denied memory is absent from selected context.
- Raw retrieved text is confined to allowed Knowledge/Context Plane fields.
- Raw parser output is confined to allowed Knowledge/Context Plane fields.
