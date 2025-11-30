# 50 – Data Plane (Arrow Spine) – Index

This folder defines the canonical data ontology of Goni. All other docs (20-architecture, 30-conformance, 95-theory) assume this as the single source of truth for tables, IDs, and invariants.

## Files

- `10-axioms-and-planes.md` – The SMA/ZCO/TXT axioms, plane partitioning (𝒜, 𝒳, 𝒦, ℰ), and the v1.0 table set.
- `20-spine-and-ids.md` – Universal `Spine` struct, UUIDv7 semantics, tenant tagging, and domain ID newtypes.
- `30-plane-contracts.md` – Plane responsibilities, allowed foreign keys, and forbidden field types per plane.
- `40-privacy-and-text-confinement.md` – The Text Confinement Theorem and privacy corollaries.
- `51-schemas-mvp.md` – Canonical Arrow schemas for the 8 MVP tables (Docs, Chunks, Embeddings, Requests, Tasks, ContextItems, LlmCalls, Metrics).
- `52-zero-copy-mechanics.md` – How submodular selection, Lyapunov scheduling, and router regret operate over Arrow buffers.
- `53-schema-dsl-and-macros.md` – `define_tables!` DSL + codegen/Clippy guards that make the schemas executable.
- `80-validation-and-ci.md` – Compile-time and CI enforcement of SMA/ZCO/TXT and schema drift gates.
- `99-changelog.md` – Versioned history of the ontology.
