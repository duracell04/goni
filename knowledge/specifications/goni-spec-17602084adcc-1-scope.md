---
id: GONI-SPEC-17602084ADCC
title: 1. Scope
type: specification
status: draft
implementation_state: specified_only
proposition: CGG-01 applies when retrieval uses graph structure to assemble Context Plane material for an LLM call, tool-mediated action, reconstruction preview, or delegated Work Order.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 1. Scope
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 1. Scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope

CGG-01 applies when retrieval uses graph structure to assemble Context Plane
material for an LLM call, tool-mediated action, reconstruction preview, or
delegated Work Order.

It defines:

- how existing canonical rows act as graph nodes,
- the specified-only future `KnowledgeGraphEdges` table concept,
- the specified-only `ContextPack` artifact produced for one Work Order,
- the scoring inputs for temporal salience,
- the context assembly flow from Work Order to `ContextItems`,
- the receipt metadata required to audit graph-influenced retrieval.

It does not define a concrete graph query language, graph database, embedding
model, UI visualization, or autonomous edge-mining implementation.
