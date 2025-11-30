# 80 – Validation & CI

How SMA/ZCO/TXT and schema drift are enforced in practice.

## 1. Compile-Time Guards
- `define_tables!` macro rejects `LargeUtf8` in planes 𝒦/ℰ and enforces plane/table pairing.
- Clippy lint `forbid(non_arrow_entity)` to block non-Arrow domain structs crossing crate boundaries.
- Newtypes for IDs prevent accidental use of raw UUIDs across APIs.

## 2. Schema Tests
- Unit test walks all generated Arrow `Schema` objects, asserting: plane tags match file placement, IDs are `FixedSizeBinary(16)`, and text confinement holds.
- Round-trip IPC/Parquet tests ensure forward compatibility (additive-only evolution).

## 3. CI Gates
- `cargo deny`/`cargo clippy --deny warnings` on `goni-schema`.
- Schema registry diff (JSON export) must be approved for any PR touching `50-data` or `goni-schema`.
- Docs/tests linkage: a change to `51-schemas-mvp.md` must accompany an update to `53-schema-dsl-and-macros.md`.

## 4. Evolution Policy
- Additive-only column changes; defaults to null for new fields.
- `schema_version` bump on semantic changes; old binaries must read new files.
- Dictionary expansions are allowed without structural change.
