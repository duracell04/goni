---
id: GONI-IMAP-2FED725A3B23
title: Retrieval
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Status: specified only / roadmap Unified retrieval API for dense, sparse, hybrid, and graph search.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/retrieval/README.md
  heading: Retrieval
  revision: 0df3d5e55823b1eaf13bbac392bcfed6967765b4
---

# Retrieval

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Retrieval

Status: specified only / roadmap

Unified retrieval API for dense, sparse, hybrid, and graph search.

Normative contract:
- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md)

Roadmap note:
- Retrieval remains the default evidence-selection baseline.
- Graph traversal is a governed retrieval signal for context assembly, not a
  replacement for Work Order binding, policy filtering, reranking, or receipts.
- CGG-01 defines ContextPack assembly: the compiled context bundle that records
  selected context, omitted candidates, compression policy, and receipt refs.
- A separate research lane may compare retrieval against programmatic
  long-context reading and hybrid retrieval + reading strategies.
- That comparison does not imply that retrieval is deprecated or replaced.
