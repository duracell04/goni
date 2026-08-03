---
id: GONI-SPEC-009C1109395A
title: 1. Scope
type: specification
status: draft
implementation_state: specified_only
proposition: This spec applies to durable personal memory, project memory, policy memory, and retrieved evidence used to assemble Context Plane material.
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
  heading: 1. Scope
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 1. Scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope

This spec applies to durable personal memory, project memory, policy memory,
and retrieved evidence used to assemble Context Plane material.

It does not define a concrete vector database, embedding model, or graph
backend. Backends are swappable if they preserve the contract below.
