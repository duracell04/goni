---
id: GONI-PRINCIPLE-0147896F31C8
title: '1.1 Memory Plane extension: latent state contract (LSC-01)'
type: principle
status: draft
implementation_state: specified_only
proposition: The Memory Plane is an operational abstraction over Knowledge/Context storage.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/10-axioms-and-planes.md
  heading: '1.1 Memory Plane extension: latent state contract (LSC-01)'
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 1.1 Memory Plane extension: latent state contract (LSC-01)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1 Memory Plane extension: latent state contract (LSC-01)

The Memory Plane is an operational abstraction over Knowledge/Context storage.
It stores kernel-owned latent artifacts as first-class payload types:

- `S_core`: dense working state (hot).
- `Delta`: append-only deltas for reconstruction.
- `F_sparse`: keyed facts/flags (typed, symbolic).
- `StateSnapshot`, `StateDelta`, `LatentSummary` records.

These artifacts MUST satisfy LSC-01:
- provenance (source, time, permissions),
- auditability (agent, policy, state snapshot),
- bounded retention and write budgets.

See `blueprint/30-specs/latent-state-contract.md` for the canonical contract.
