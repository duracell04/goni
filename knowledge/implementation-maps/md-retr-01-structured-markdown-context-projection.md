---
id: MD-RETR-01
title: Structured Markdown Context Projection
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni should project Markdown knowledge into retrieval and context systems through its explicit document structure, frontmatter, permanent IDs, headings, relations, and source metadata before applying arbitrary token-length segmentation.
domains:
- context
- retrieval
- repository
aliases:
- markdown retrieval projection
relations:
- type: refines
  target: MEM-RETR-01
- type: depends_on
  target: GONI-SPEC-B49DB23CF412
- type: depends_on
  target: TRUST-INPUT-01
sources: []
artifacts: []
uncertainty: Heading-aware and proposition-aware segmentation is an implementation hypothesis that should be evaluated against alternative chunking strategies; Markdown syntax itself is not assumed to improve model intelligence.
legacy: []
---

# Structured Markdown Context Projection

Goni's canonical Markdown knowledge graph already carries machine-visible structure that should be preserved through retrieval.

The intended projection is:

```text
Markdown node
-> YAML frontmatter
-> permanent node identity / type / status / relations / sources
-> heading and section structure
-> semantic retrieval units
-> dense / sparse / exact / graph indexes
-> governed candidate selection
-> ContextPack
```

## Structural parsing before token slicing

Retrieval preprocessing should first identify meaningful document boundaries such as:

- canonical node;
- proposition/frontmatter;
- heading and subsection;
- source-backed claim;
- decision/rationale block;
- invariant or contract section;
- table or list where row/entry integrity matters.

Token limits may then subdivide oversized units while preserving parent node, heading path, source refs, and positional metadata.

Fixed token windows remain a baseline to evaluate rather than the default semantic abstraction.

## Metadata

Each indexed unit should retain, directly or by stable ref:

- canonical node ID;
- node type and status;
- implementation state;
- title and heading path;
- domain tags;
- relation waypoints;
- source refs;
- legacy provenance where relevant;
- document revision or content hash;
- trust/provenance class; and
- retrieval permissions.

This allows retrieval to combine semantic similarity with exact identifiers, symbolic filters, source trust, relation traversal, status, and provenance.

## Authority boundary

Retrieved Markdown enters the Context Plane as information. Its imperative wording does not grant instruction privilege.

A canonical policy or WorkOrder constraint may be loaded through an authorized instruction path. A research note, archived AGENTS.md, quoted prompt, issue body, external README, or arbitrary retrieved Markdown remains data unless its provenance and current control-plane role explicitly grant authority.

This preserves `TRUST-INPUT-01` across repository RAG.

## Evaluation

Compare structured projection with token-only chunking using:

- retrieval recall for exact decisions/invariants;
- source recovery;
- contradiction and supersession handling;
- citation precision;
- context tokens required;
- answer/task success;
- prompt-injection resistance; and
- ingestion/indexing overhead.

The purpose is to exploit existing repository structure when it improves retrieval and auditability, not to claim that Markdown formatting itself is intrinsically superior for language models.
