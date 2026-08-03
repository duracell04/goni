---
id: GONI-IMAP-1C03C56C4C2C
title: Files
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 10-axioms-and-planes.md – The SMA/ZCO/TXT axioms, plane partitioning (𝒜, 𝒳, 𝒦, ℰ), and the v1.0 table set.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/00-index.md
  heading: Files
  revision: 35717f31df1deb53ae7d83fe62cbcf6e487fdb4b
---

# Files

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Files

- `10-axioms-and-planes.md` – The SMA/ZCO/TXT axioms, plane partitioning (𝒜, 𝒳, 𝒦, ℰ), and the v1.0 table set.
- `20-spine-and-ids.md` – Universal `Spine` struct, UUIDv7 semantics, tenant tagging, and domain ID newtypes.
- `30-plane-contracts.md` – Plane responsibilities, allowed foreign keys, and forbidden field types per plane.
- `40-privacy-and-text-confinement.md` – The Text Confinement Theorem and privacy corollaries.
- `51-schemas-mvp.md` - Canonical Arrow schemas for the MVP tables, including latent state, visual assets, audit, capabilities, and agent manifests.
- `52-zero-copy-mechanics.md` – How submodular selection, Lyapunov scheduling, and router regret operate over Arrow buffers.
- `53-schema-dsl-and-macros.md` – `define_tables!` DSL + codegen/Clippy guards that make the schemas executable.
- `80-validation-and-ci.md` – Compile-time and CI enforcement of SMA/ZCO/TXT and schema drift gates.
- `99-changelog.md` – Versioned history of the ontology.
