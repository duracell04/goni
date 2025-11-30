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
