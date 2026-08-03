---
id: GONI-SPEC-3EDF04F2E827
title: 6. Identity Resolution
type: specification
status: draft
implementation_state: specified_only
proposition: CGG-01 distinguishes identity from relation.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 6. Identity Resolution
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 6. Identity Resolution

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Identity Resolution

CGG-01 distinguishes identity from relation. Similar phrases may describe the
same concept, aliases for one object, or merely related ideas.

Graph implementations SHOULD preserve:

- canonical IDs for durable concepts, projects, people, artifacts, and memory
  entries,
- aliases and surface forms,
- duplicate-detection evidence,
- concept cluster refs,
- merge/split provenance and undo refs.

Merge operations MUST preserve source refs and receipts. Split operations MUST
preserve prior aliases and explain why one cluster became multiple concepts.
Concept clusters may influence salience through `same_theme_as`, `refines`, or
`applies_to` edges, but they do not override canonical node identity.

The ontology MUST be no broader than necessary for retrieval, permission,
temporal reasoning, and principal-directed action. Merge and split operations
MUST preserve dissent, rationale, prior identities, and undo refs. Inferred
relationships MUST remain distinguishable from principal-set or imported
explicit relationships. No merge, cluster score, or canonical label creates
truth or operational authority by itself: the map is not the territory.
