# 53 – Schema DSL & Codegen Macros (goni-schema)

**Goal:** The code is the spec. The `goni-schema` crate owns a `define_tables!` DSL that emits Arrow `Schema` definitions, typed batch wrappers, and compile-time guards for SMA/ZCO/TXT.

## 1. DSL Shape (illustrative)

```rust
define_tables! {
    table Docs {
        plane: Plane::Knowledge,
        kind: "Docs",
        fields: {
            doc_id: FixedSizeBinary(16),
            source_uri: LargeUtf8,
            mime_type: Utf8,
            title: Utf8,
            tags: List<Utf8>,
            metadata: Map<Utf8, Utf8>,
        }
    },

    table Requests {
        plane: Plane::Control,
        kind: "Requests",
        fields: {
            request_id: FixedSizeBinary(16),
            session_id: FixedSizeBinary(16),
            task_class: Dict(UInt8, Utf8),
            prompt_hash: FixedSizeBinary(32),
            budget_tokens: UInt32,
        }
    },

    // ... remaining 6 MVP tables ...
}
```

## 2. Macro Guarantees
- SMA: Only tables declared in this block are canonical; missing entries fail build.
- TXT: If `plane == Control || plane == Execution` and any field is `LargeUtf8`, compilation fails.
- ZCO: For each table, a `*Batch` type wraps `Arc<RecordBatch>`; public APIs must traffic in these types or opaque IDs.
- Plane enforcement: `plane` tag is fixed per table; mismatches are rejected.

## 3. Mapping to Human Docs
- The DSL content is mechanically identical to `51-schemas-mvp.md`; that document is human-readable, this one is machine-checked.
- Any schema change requires updating both: the DSL definition here and the narrative/table form in `51-schemas-mvp.md`.

## 4. Outputs
- Generated Arrow `Schema` objects (for IPC/Parquet writers).
- Generated Rust newtypes for IDs (e.g., `DocId`, `TaskId`).
- Generated typed batches (e.g., `RequestsBatch`) with field accessors.
- Optional: schema registry JSON for non-Rust consumers.
