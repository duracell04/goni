# 10 – Axioms & Planes (Arrow Spine Constitution)
DOC-ID: AXIOMS-01
**Version:** v1.0 (2027-04-01)
**Status:** Normative, CI-enforced

## 0. Axioms

| Axiom | Name | Statement | Enforcement |
|------|------|-----------|-------------|
| SMA  | Single-Model Axiom | Every persistent or transient entity is exactly one row in a canonical table. | Schema registry + `goni-schema` + Clippy lint |
| ZCO  | Zero-Copy Ontology | Cross-crate APIs expose only Arrow batches + opaque IDs (`RecordBatch`/`Arc<RecordBatch>`). | `forbid(non_arrow_entity)` lint + API review gate |
| TXT  | Text Confinement | Raw text (`LargeUtf8` > 512 bytes) exists only in `Chunks.text` and `Prompts.text`. | Schema validator + macro guard |

Violation of any axiom is a compile-time error in CI.

## 1. Planes (partitioning)

| Plane | Symbol | Lifetime | Sensitivity | Storage | Primary Keys |
|-------|--------|----------|-------------|---------|--------------|
| Knowledge | 𝒜 | Permanent | High (raw text) | Parquet + Lance v2 | `doc_id`, `chunk_id`, `embedding_id` |
| Context | 𝒳 | ≤ 24 h | High (live prompts) | Memory + optional spill to encrypted tmp | `request_id`, `context_id` |
| Control | 𝒦 | Permanent | Low (metadata only) | WAL + Parquet | `request_id`, `task_id` |
| Execution | ℰ | Permanent | Low (aggregates) | Parquet (append-only) | `span_id`, `call_id` |

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

See `docs/specs/latent-state-contract.md` for the canonical contract.

## 2. v1.0 Table Set (ships in binary)

We ship the following canonical tables in v1.0:

1. Docs
2. Chunks
3. Embeddings
4. Requests
5. Tasks
6. ContextItems
7. LlmCalls
8. Metrics
9. StateSnapshots
10. StateDeltas
11. LatentSummaries
12. AuditRecords
13. CapabilityTokens
14. AgentManifests

Any new canonical table must be added to the schema DSL (see
`53-schema-dsl-and-macros.md`) and documented in `51-schemas-mvp.md`.
