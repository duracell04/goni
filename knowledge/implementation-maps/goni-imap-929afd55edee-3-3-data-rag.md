---
id: GONI-IMAP-929AFD55EDEE
title: 3.3 Data & RAG
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Gives a mental model of “what the node knows” and how RAG behaves: Connected sources (file roots, mailboxes, etc.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 3.3 Data & RAG
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.3 Data & RAG

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Data & RAG

Gives a mental model of “what the node knows” and how RAG behaves:

- Connected sources (file roots, mailboxes, etc. – even if conceptual in MVP).
- Counts:
  - documents,
  - chunks,
  - index sizes.
- For a sample query:
  - retrieved candidates from VecDB (with similarity),
  - selected subset \(S\) used in the actual context.

**Source:** VecDB + Context Plane \(\mathcal{X}\) logging.

> **Invariant UI-3 (context transparency)**  
> Any explanation like “this chunk was selected because …” must be backed by actual data from:
> - VecDB search results (similarity scores), and
> - the selector’s choice set \(S\) and objective contributions.  
> The UI cannot invent explanations that the kernel cannot justify.

---
