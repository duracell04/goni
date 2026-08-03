---
id: GONI-EVIDENCE-43DF2222EE37
title: 'Source claim: reimers2019-sbert'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Sentence-BERT uses siamese/triplet structures to produce sentence embeddings that can be compared efficiently for semantic similarity search.
domains:
- research
aliases: []
relations:
- type: supports
  target: MEM-RETR-01
sources:
- SRC-REIMERS2019-SBERT
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[reimers2019-sbert]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: reimers2019-sbert

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[reimers2019-sbert]]
Claim: Sentence-BERT uses siamese/triplet structures to produce sentence
embeddings that can be compared efficiently for semantic similarity search.
Relevance:
- Supports dense semantic retrieval over user-owned chunks.
- Distinguishes meaning search from exact keyword lookup.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/1908.10084
