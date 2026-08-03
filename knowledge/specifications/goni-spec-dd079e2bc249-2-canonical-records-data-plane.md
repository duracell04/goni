---
id: GONI-SPEC-DD079E2BC249
title: 2. Canonical records (data plane)
type: specification
status: draft
implementation_state: specified_only
proposition: 'The LSS uses these canonical tables (see blueprint/software/50-data/51-schemas-mvp.md and blueprint/software/50-data/53-schema-dsl-and-macros.md): StateSnapshots: point-in-time snapshots of S_core + F_sparse.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 2. Canonical records (data plane)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 2. Canonical records (data plane)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Canonical records (data plane)

The LSS uses these canonical tables (see `blueprint/software/50-data/51-schemas-mvp.md`
and `blueprint/software/50-data/53-schema-dsl-and-macros.md`):

- `StateSnapshots`: point-in-time snapshots of `S_core` + `F_sparse`.
- `StateDeltas`: append-only deltas applied to a snapshot.
- `LatentSummaries`: compact derived summaries (optional, budgeted).

These records are immutable once written.
