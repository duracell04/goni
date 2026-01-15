# 99 – Data Ontology Changelog

## v1.0 – 2027-04-01
- Introduced SMA/ZCO/TXT axioms.
- Finalized 4-plane model (𝒜, 𝒳, 𝒦, ℰ).
- Defined 8 MVP tables: Docs, Chunks, Embeddings, Requests, Tasks, ContextItems, LlmCalls, Metrics.
- Added schema DSL concept in `goni-schema` (see `53-schema-dsl-and-macros.md`).

## v1.1 – 2026-01-15
- Added Prompts table and materialization metadata (hash, redaction flags).
- Added RedactionProfiles and RedactionEvents tables for minimization/audit.
- Added MemoryEntries table with gating fields (kind, confidence, review/ttl).
