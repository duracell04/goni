---
id: GONI-SPEC-E1E785A04953
title: 2. Nodes
type: specification
status: draft
implementation_state: specified_only
proposition: Graph nodes are existing rows in canonical Arrow tables.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 2. Nodes
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 2. Nodes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Nodes

Graph nodes are existing rows in canonical Arrow tables. A graph node reference
MUST identify the table kind and row ID. Eligible node kinds include, but are
not limited to:

- Knowledge rows: `Docs`, `Chunks`, `Embeddings`, `MemoryEntries`,
  `StateSnapshots`, `StateDeltas`, and `LatentSummaries`.
- Context rows: `Prompts` and `ContextItems`.
- Control rows: `Requests`, `Tasks`, `WorkOrders`, `AuditRecords`,
  `CapabilityTokens`, and `AgentManifests`.
- Execution rows when relevant as evidence or telemetry waypoints:
  `LlmCalls`, `PlatformSignals`, `PlatformCapabilities`, and `Metrics`.

Node content authority comes from the underlying table and its policy metadata.
CGG-01 MUST NOT create a second source of truth for node text, permissions, or
retention state.
