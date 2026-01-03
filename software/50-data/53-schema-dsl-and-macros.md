# 53 – Schema DSL & Codegen Macros (goni-schema)
DOC-ID: SCHEMA-DSL-01

**Goal:** The code is the spec. The `goni-schema` crate owns a `define_tables!` DSL that emits Arrow `Schema` definitions, typed batch wrappers, and compile-time guards for SMA/ZCO/TXT.

## 1. DSL Tokens (frozen v1.0)

- Planes: `Plane::Knowledge | Plane::Context | Plane::Control | Plane::Execution`
- Types: `FixedSizeBinary(16)`, `Utf8`, `LargeUtf8`, `DictU8Utf8`, `ListUtf8`, `MapUtf8Utf8`, `Int32`, `UInt32`, `Int64`, `UInt16`, `UInt8`, `Float32`, `Float64`, `Boolean`, `FixedSizeListF32(N)`, `TimestampMsUtc`.

## 2. DSL Shape (illustrative)

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

    // ... remaining tables ...
}
```

## 3. MVP Tables (authoritative DSL block)

This block must match `51-schemas-mvp.md` and is pasted into `goni-schema/src/lib.rs`:

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
            tags: ListUtf8,
            metadata: MapUtf8Utf8,
        }
    },

    table Chunks {
        plane: Plane::Knowledge,
        kind: "Chunks",
        fields: {
            chunk_id: FixedSizeBinary(16),
            doc_id: FixedSizeBinary(16),
            ordinal: UInt32,
            text: LargeUtf8,
            token_count: UInt32,
            section_path: ListUtf8,
        }
    },

    table Embeddings {
        plane: Plane::Knowledge,
        kind: "Embeddings",
        fields: {
            embedding_id: FixedSizeBinary(16),
            chunk_id: FixedSizeBinary(16),
            model_id: DictU8Utf8,
            vector: FixedSizeListF32(1536),
            dim: UInt16,
        }
    },

    table Requests {
        plane: Plane::Control,
        kind: "Requests",
        fields: {
            request_id: FixedSizeBinary(16),
            session_id: FixedSizeBinary(16),
            prompt_hash: FixedSizeBinary(32),
            prompt_tokens_est: UInt32,
            budget_tokens: UInt32,
            task_class: DictU8Utf8,
        }
    },

    table Tasks {
        plane: Plane::Control,
        kind: "Tasks",
        fields: {
            task_id: FixedSizeBinary(16),
            request_id: FixedSizeBinary(16),
            task_type: DictU8Utf8,
            state: DictU8Utf8,
            queue_id: DictU8Utf8,
            expected_cost_tokens: UInt32,
        }
    },

    table AuditRecords {
        plane: Plane::Control,
        kind: "AuditRecords",
        fields: {
            audit_id: FixedSizeBinary(16),
            agent_id: FixedSizeBinary(16),
            policy_hash: FixedSizeBinary(32),
            state_snapshot_id: FixedSizeBinary(16),
            capability_token_id: FixedSizeBinary(16),
            tool_id: DictU8Utf8,
            args_hash: FixedSizeBinary(32),
            result_hash: FixedSizeBinary(32),
            timestamp: TimestampMsUtc,
            provenance: MapUtf8Utf8,
        }
    },

    table CapabilityTokens {
        plane: Plane::Control,
        kind: "CapabilityTokens",
        fields: {
            capability_token_id: FixedSizeBinary(16),
            agent_id: FixedSizeBinary(16),
            policy_hash: FixedSizeBinary(32),
            tools: ListUtf8,
            fs_read_roots: ListUtf8,
            fs_write_roots: ListUtf8,
            net_allowlist: ListUtf8,
            budgets: MapUtf8Utf8,
            issued_at: TimestampMsUtc,
            expires_at: TimestampMsUtc,
            provenance: MapUtf8Utf8,
        }
    },

    table AgentManifests {
        plane: Plane::Control,
        kind: "AgentManifests",
        fields: {
            manifest_id: FixedSizeBinary(16),
            agent_id: FixedSizeBinary(16),
            version: Utf8,
            manifest_hash: FixedSizeBinary(32),
            manifest_uri: Utf8,
            triggers: MapUtf8Utf8,
            capabilities: MapUtf8Utf8,
            budgets: MapUtf8Utf8,
            ui_surfaces: ListUtf8,
            identity_requirements: ListUtf8,
            remote_access: Boolean,
            tools: ListUtf8,
            policy_hash: FixedSizeBinary(32),
            state_snapshot_id: FixedSizeBinary(16),
            provenance: MapUtf8Utf8,
        }
    },

    table ContextItems {
        plane: Plane::Context,
        kind: "ContextItems",
        fields: {
            context_item_id: FixedSizeBinary(16),
            context_id: FixedSizeBinary(16),
            chunk_id: FixedSizeBinary(16),
            cost_tokens: UInt32,
            selected: Boolean,
            rank: UInt16,
            marginal_gain: Float32,
        }
    },

    table StateSnapshots {
        plane: Plane::Knowledge,
        kind: "StateSnapshots",
        fields: {
            snapshot_id: FixedSizeBinary(16),
            state_version: UInt32,
            s_core: FixedSizeListF32(1536),
            s_core_dim: UInt16,
            f_sparse: MapUtf8Utf8,
            created_at: TimestampMsUtc,
            agent_id: FixedSizeBinary(16),
            policy_hash: FixedSizeBinary(32),
            state_snapshot_id: FixedSizeBinary(16),
            provenance: MapUtf8Utf8,
        }
    },

    table StateDeltas {
        plane: Plane::Knowledge,
        kind: "StateDeltas",
        fields: {
            delta_id: FixedSizeBinary(16),
            snapshot_id: FixedSizeBinary(16),
            delta_kind: DictU8Utf8,
            delta_vector: FixedSizeListF32(1536),
            delta_dim: UInt16,
            f_sparse_patch: MapUtf8Utf8,
            timestamp: TimestampMsUtc,
            agent_id: FixedSizeBinary(16),
            policy_hash: FixedSizeBinary(32),
            state_snapshot_id: FixedSizeBinary(16),
            provenance: MapUtf8Utf8,
        }
    },

    table LatentSummaries {
        plane: Plane::Knowledge,
        kind: "LatentSummaries",
        fields: {
            summary_id: FixedSizeBinary(16),
            snapshot_id: FixedSizeBinary(16),
            summary_kind: DictU8Utf8,
            summary_vector: FixedSizeListF32(1536),
            summary_dim: UInt16,
            summary_hash: FixedSizeBinary(32),
            timestamp: TimestampMsUtc,
            agent_id: FixedSizeBinary(16),
            policy_hash: FixedSizeBinary(32),
            state_snapshot_id: FixedSizeBinary(16),
            provenance: MapUtf8Utf8,
        }
    },

    table LlmCalls {
        plane: Plane::Execution,
        kind: "LlmCalls",
        fields: {
            call_id: FixedSizeBinary(16),
            request_id: FixedSizeBinary(16),
            model_id: DictU8Utf8,
            prompt_tokens: UInt32,
            completion_tokens: UInt32,
            total_tokens: UInt32,
            latency_ms: UInt32,
            cache_hit: Boolean,
        }
    },

    table Metrics {
        plane: Plane::Execution,
        kind: "Metrics",
        fields: {
            metric_id: FixedSizeBinary(16),
            name: DictU8Utf8,
            value_float: Float64,
            value_int: Int64,
            labels: MapUtf8Utf8,
        }
    }
}
```

## 4. Macro Guarantees
- SMA: Only tables declared in this block are canonical; missing entries fail build.
- TXT: If `plane == Control || plane == Execution` and any field is `LargeUtf8`, compilation fails.
- ZCO: For each table, a `*Batch` type wraps `Arc<RecordBatch>`; public APIs must traffic in these types or opaque IDs.
- Plane enforcement: `plane` tag is fixed per table; mismatches are rejected.

## 5. Mapping to Human Docs
- The DSL content is mechanically identical to `51-schemas-mvp.md`; that document is human-readable, this one is machine-checked.
- Any schema change requires updating both: the DSL definition here and the narrative/table form in `51-schemas-mvp.md`.

## 6. Outputs
- Generated Arrow `Schema` objects (for IPC/Parquet writers).
- Generated Rust newtypes for IDs (e.g., `DocId`, `TaskId`).
- Generated typed batches (e.g., `RequestsBatch`) with field accessors.
- Optional: schema registry JSON for non-Rust consumers.
