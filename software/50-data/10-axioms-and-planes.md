# 10 – Axioms & Planes (Arrow Spine Constitution)
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

### 1.1 Memory Plane extension: latent state + latent summaries (optional)

The Memory Plane is an operational abstraction over Knowledge/Context storage. It may store **latent state artifacts** as non-canonical records, subject to the axioms (SMA, ZCO, TXT):

- **State embedding**: a compact vector representing "what is going on" for a session/thread.
- **Latent summary record**: a time-stamped summary derived from state, with provenance.

These artifacts are optional and must respect the same guarantees as other memory:
- provenance (source, time, permissions),
- auditability (what inputs contributed),
- bounded retention (expiry / rotation policy).

If promoted to canonical tables, they must be added to the schema DSL and the v1.0 table set.

## 2. v1.0 Table Set (ships in binary)

We ship exactly eight canonical tables in v1.0:

1. Docs
2. Chunks
3. Embeddings
4. Requests
5. Tasks
6. ContextItems
7. LlmCalls
8. Metrics

Any new canonical table must be added to the schema DSL (see `53-schema-dsl-and-macros.md`) and documented in `51-schemas-mvp.md`.
