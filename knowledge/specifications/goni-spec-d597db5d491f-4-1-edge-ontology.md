---
id: GONI-SPEC-D597DB5D491F
title: 4.1 Edge Ontology
type: specification
status: draft
implementation_state: specified_only
proposition: Edge types are controlled because they affect salience differently.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 4.1 Edge Ontology
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 4.1 Edge Ontology

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Edge Ontology

Edge types are controlled because they affect salience differently. The minimum
ontology is:

| Edge type | Salience semantics |
| --- | --- |
| `supports` | Positive evidence for a claim, decision, or context candidate. |
| `contradicts` | Negative or competing evidence that SHOULD be retrievable and surfaced as uncertainty, not silently ignored. |
| `supersedes` | Shifts authority toward the source or target marked as newer by provenance and validity metadata. |
| `refines` | Narrows or improves a prior node without fully replacing it. |
| `depends_on` | Pulls prerequisite context when the current Work Order needs the dependent node. |
| `inspired_by` | Weak creative or conceptual affinity; useful for ideation, lower weight for factual tasks. |
| `same_theme_as` | Cluster-level thematic similarity; useful for recall expansion but weaker than evidence edges. |
| `applies_to` | Scopes a memory, policy, decision, or skill to a project, person, task, or Work Order. |
| `blocks` | Indicates a constraint or unresolved issue that can prevent use or require surfacing. |
| `derived_from` | Provenance edge from source material to derived memory, summary, decision, or artifact. |

Implementations MAY add labels only through a schema/spec revision or controlled
configuration with stable semantics. `supports` and `contradicts` may both
increase retrieval priority, but they MUST NOT be treated as equivalent during
reranking or prompt assembly.
