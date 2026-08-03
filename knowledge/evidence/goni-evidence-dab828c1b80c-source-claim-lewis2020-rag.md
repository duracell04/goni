---
id: GONI-EVIDENCE-DAB828C1B80C
title: 'Source claim: lewis2020-rag'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Retrieval-augmented generation separates parametric model memory from non-parametric retrieved evidence, improving knowledge-intensive generation and making retrieved sources part of the generation path.
domains:
- research
aliases: []
relations:
- type: supports
  target: MEM-RETR-01
sources:
- SRC-LEWIS2020-RAG
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[lewis2020-rag]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: lewis2020-rag

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[lewis2020-rag]]
Claim: Retrieval-augmented generation separates parametric model memory from
non-parametric retrieved evidence, improving knowledge-intensive generation
and making retrieved sources part of the generation path.
Relevance:
- Supports external, updateable memory rather than storing personal knowledge
  only in model weights.
- Grounds Goni's Work Order driven retrieval plane.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/2005.11401
