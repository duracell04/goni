---
id: GONI-SPECIFICATION-C64D8275BA34
title: Cognitive Memory Residency Contract
type: specification
status: draft
implementation_state: specified_only
proposition: Cognitive memory class and residency are orthogonal dimensions, and changes in residency may change retrieval cost without silently changing epistemic status, provenance, retention authority, or execution authority.
domains:
- memory
- architecture
aliases: []
relations:
- type: refines
  target: GONI-THESIS-AEA4F8746318
- type: depends_on
  target: GONI-SYNTHESIS-2F274E6CBCF8
sources:
- SRC-KWON2023-PAGEDATTENTION
- SRC-JUSTVUGG2026-COLIBRI
artifacts: []
uncertainty: The tier boundaries and promotion or demotion policies are specified abstractions. Concrete thresholds require measurement on target hardware and workloads.
legacy: []
---

# Cognitive Memory Residency Contract

For a memory item `m`, GONI models at least two independent dimensions:

`m = (semantic_class, residency)`

The semantic class describes the item's role in the cognitive system. Residency describes how immediately the item can be supplied to inference. Residency is therefore a performance and availability property, not a truth or permission label.

A provisional residency hierarchy is:

- `L0 Active`: material currently supplied to attention or represented in active inference state.
- `L1 Pinned`: task-critical state that should remain immediately recoverable, including the current objective, mandate, approved constraints, invariants, and unresolved blockers.
- `L2 Hot`: frequently relevant structured state such as current project decisions, entities, recent episodes, and dependency context.
- `L3 Warm Exact`: exact prior turns, documents, tool outputs, source passages, and optionally persisted inference state that can be rehydrated when needed.
- `L4 Canonical Archive`: durable source artifacts, receipts, files, Git state, databases, and other authoritative or reconstructable system state.

A transition `L_i → L_j` may alter expected latency, context cost, memory footprint, or recomputation cost. It must not, by itself, change whether a claim is true, whether a source is authoritative, whether provenance exists, whether retention is permitted, or whether an action is authorized.

Accordingly:

**Placement should determine latency, not truth.**

Demotion is preferred to destructive forgetting when the information remains worth retaining and the marginal storage cost is below the expected value of future recovery. Promotion should be driven by expected near-term utility, criticality, dependency structure, and retrieval cost rather than recency alone.
