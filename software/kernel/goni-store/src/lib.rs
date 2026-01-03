use std::sync::Arc;

use arrow::record_batch::RecordBatch;
use async_trait::async_trait;
use thiserror::Error;

pub mod qdrant;
pub use qdrant::QdrantDataPlane;

pub mod spine_mem;
pub use spine_mem::InMemorySpineDataPlane;

pub mod multi;
pub use multi::MultiDataPlane;

pub type ArrowBatch = RecordBatch;
pub type ArrowBatchHandle = Arc<ArrowBatch>;

#[derive(Debug, Error)]
#[error("data plane error: {message}")]
pub struct DataError {
    pub message: String,
}

/// The Arrow Spine: all structured data flows through this trait.
#[async_trait]
pub trait DataPlane: Send + Sync {
    /// Run a SQL-like query (DuckDB/DataFusion) and return Arrow batches.
    async fn query(
        &self,
        sql: &str,
    ) -> Result<Vec<ArrowBatchHandle>, DataError>;

    /// Append batches into a logical table.
    async fn append_batches(
        &self,
        table: &str,
        batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError>;

    /// RAG/ANN query: return top-k candidate chunks with embeddings.
    async fn rag_candidates(
        &self,
        collection: &str,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<ArrowBatchHandle, DataError>;
}

/// Stub implementation for now – replace with DuckDB + LanceDB.
pub struct NullDataPlane;

#[async_trait]
impl DataPlane for NullDataPlane {
    async fn query(
        &self,
        _sql: &str,
    ) -> Result<Vec<ArrowBatchHandle>, DataError> {
        Ok(Vec::new())
    }

    async fn append_batches(
        &self,
        _table: &str,
        _batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError> {
        Ok(())
    }

    async fn rag_candidates(
        &self,
        _collection: &str,
        _query_embedding: &[f32],
        _top_k: usize,
    ) -> Result<ArrowBatchHandle, DataError> {
        Err(DataError {
            message: "NullDataPlane has no RAG".into(),
        })
    }
}
