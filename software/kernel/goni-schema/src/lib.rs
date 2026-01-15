#![forbid(unsafe_code)]

pub mod plane;
pub mod macros;

use plane::Plane;

/// Generated table wrappers and schemas based on the MVP tables (see software/50-data/51-schemas-mvp.md).
pub mod generated {
    use super::Plane;
    use crate::define_tables;

    define_tables! {
        // Plane A - Docs
        table Docs {
            plane: Plane::Knowledge,
            kind: "Docs",
            fields: {
                doc_id: FixedSizeBinary(16),
                source_uri: LargeUtf8,
                mime_type: Utf8,
                title: Utf8,
                tags: ListUtf8,
                metadata: MapUtf8Utf8
            }
        },

        // Plane A - Chunks
        table Chunks {
            plane: Plane::Knowledge,
            kind: "Chunks",
            fields: {
                chunk_id: FixedSizeBinary(16),
                doc_id: FixedSizeBinary(16),
                ordinal: UInt32,
                text: LargeUtf8,
                token_count: UInt32,
                section_path: ListUtf8
            }
        },

        // Plane A - Embeddings
        table Embeddings {
            plane: Plane::Knowledge,
            kind: "Embeddings",
            fields: {
                embedding_id: FixedSizeBinary(16),
                chunk_id: FixedSizeBinary(16),
                model_id: DictU8Utf8,
                vector: FixedSizeListF32(1536),
                dim: UInt16
            }
        },

        // Plane K - Requests
        table Requests {
            plane: Plane::Control,
            kind: "Requests",
            fields: {
                request_id: FixedSizeBinary(16),
                session_id: FixedSizeBinary(16),
                prompt_hash: FixedSizeBinary(32),
                prompt_tokens_est: UInt32,
                budget_tokens: UInt32,
                task_class: DictU8Utf8
            }
        },

        // Plane K - Tasks
        table Tasks {
            plane: Plane::Control,
            kind: "Tasks",
            fields: {
                task_id: FixedSizeBinary(16),
                request_id: FixedSizeBinary(16),
                task_type: DictU8Utf8,
                state: DictU8Utf8,
                queue_id: DictU8Utf8,
                expected_cost_tokens: UInt32
            }
        },

        // Plane K - AuditRecords
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane K - CapabilityTokens
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane K - RedactionProfiles
        table RedactionProfiles {
            plane: Plane::Control,
            kind: "RedactionProfiles",
            fields: {
                redaction_profile_id: FixedSizeBinary(16),
                name: Utf8,
                mode: DictU8Utf8,
                ruleset_hash: FixedSizeBinary(32),
                created_at: TimestampMsUtc
            }
        },

        // Plane K - RedactionEvents
        table RedactionEvents {
            plane: Plane::Control,
            kind: "RedactionEvents",
            fields: {
                redaction_event_id: FixedSizeBinary(16),
                request_id: FixedSizeBinary(16),
                redaction_profile_id: FixedSizeBinary(16),
                timestamp: TimestampMsUtc,
                before_hash: FixedSizeBinary(32),
                after_hash: FixedSizeBinary(32),
                redaction_summary: MapUtf8Utf8
            }
        },

        // Plane K - AgentManifests
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane X - Prompts
        table Prompts {
            plane: Plane::Context,
            kind: "Prompts",
            fields: {
                prompt_id: FixedSizeBinary(16),
                request_id: FixedSizeBinary(16),
                source_context_id: FixedSizeBinary(16),
                timestamp: TimestampMsUtc,
                materialization_kind: DictU8Utf8,
                prompt_hash: FixedSizeBinary(32),
                token_estimate_in: UInt32,
                token_estimate_out: UInt32,
                is_redacted: Boolean,
                redaction_profile_id: FixedSizeBinary(16),
                text: LargeUtf8
            }
        },

        // Plane X - ContextItems
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
                marginal_gain: Float32
            }
        },

        // Plane A - StateSnapshots
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane A - StateDeltas
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane A - LatentSummaries
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
                provenance: MapUtf8Utf8
            }
        },

        // Plane A - MemoryEntries
        table MemoryEntries {
            plane: Plane::Knowledge,
            kind: "MemoryEntries",
            fields: {
                memory_id: FixedSizeBinary(16),
                kind: DictU8Utf8,
                timestamp: TimestampMsUtc,
                value: MapUtf8Utf8,
                confidence: Float32,
                source_chunk_ids: ListUtf8,
                confirmed_by_event_id: FixedSizeBinary(16),
                review_at: TimestampMsUtc,
                ttl_ms: UInt32,
                conflict_state: DictU8Utf8,
                embedding: FixedSizeListF32(1536),
                embedding_dim: UInt16
            }
        },

        // Plane E - LlmCalls
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
                cache_hit: Boolean
            }
        },

        // Plane E - PlatformSignals
        table PlatformSignals {
            plane: Plane::Execution,
            kind: "PlatformSignals",
            fields: {
                signal_id: FixedSizeBinary(16),
                timestamp: TimestampMsUtc,
                device_id: FixedSizeBinary(16),
                session_id: FixedSizeBinary(16),
                thermal_throttled: Boolean,
                thermal_domain: DictU8Utf8,
                dvfs_state: DictU8Utf8,
                free_ram_mb: UInt32,
                swap_in_mb: UInt32,
                major_faults: UInt32,
                bytes_written_today: Int64,
                waf_estimate: Float32,
                ssd_health: Float32,
                npu_shape_buckets: ListUtf8,
                supported_quant: ListUtf8,
                gpu_active: Boolean,
                gpu_wake_ms_p95: UInt32,
                solver_wake_count: UInt32,
                solver_active_ms: UInt32,
                encoder_active_ms: UInt32
            }
        },

        // Plane E - PlatformCapabilities
        table PlatformCapabilities {
            plane: Plane::Execution,
            kind: "PlatformCapabilities",
            fields: {
                capability_id: FixedSizeBinary(16),
                timestamp: TimestampMsUtc,
                device_id: FixedSizeBinary(16),
                npu_shape_buckets: ListUtf8,
                supported_quant: ListUtf8
            }
        },

        // Plane E - Metrics
        table Metrics {
            plane: Plane::Execution,
            kind: "Metrics",
            fields: {
                metric_id: FixedSizeBinary(16),
                name: DictU8Utf8,
                value_float: Float64,
                value_int: Int64,
                labels: MapUtf8Utf8
            }
        }
    }
}

pub use generated::*;
