use std::sync::Arc;

use async_trait::async_trait;

use crate::{ArrowBatchHandle, DataError, DataPlane};

/// A small router that lets us combine:
/// - a general-purpose "spine" DataPlane (append-only tables), and
/// - a RAG/ANN DataPlane (e.g. Qdrant).
///
/// This keeps the core kernel model (everything is an Arrow batch) intact during MVP,
/// while acknowledging that early RAG ingestion/query often uses a specialised backend.
pub struct MultiDataPlane {
    pub spine: Arc<dyn DataPlane>,
    pub rag: Arc<dyn DataPlane>,
}

impl MultiDataPlane {
    pub fn new(spine: Arc<dyn DataPlane>, rag: Arc<dyn DataPlane>) -> Self {
        Self { spine, rag }
    }

    fn looks_like_rag_ingest(batch: &arrow::record_batch::RecordBatch) -> bool {
        let s = batch.schema();
        // Heuristic: QdrantDataPlane expects plain utf8 columns "id" and "text", and u32 "tokens".
        s.index_of("id").is_ok() && s.index_of("text").is_ok() && s.index_of("tokens").is_ok()
            && s.index_of("row_id").is_err()
    }
}

#[async_trait]
impl DataPlane for MultiDataPlane {
    async fn query(&self, sql: &str) -> Result<Vec<ArrowBatchHandle>, DataError> {
        self.spine.query(sql).await
    }

    async fn append_batches(
        &self,
        table: &str,
        batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError> {
        if batches.iter().any(|b| Self::looks_like_rag_ingest(b.as_ref())) {
            // Route to RAG backend (Qdrant ingestion) when batch shape matches.
            self.rag.append_batches(table, batches).await
        } else {
            // Otherwise treat as an Arrow spine table.
            self.spine.append_batches(table, batches).await
        }
    }

    async fn rag_candidates(
        &self,
        collection: &str,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<ArrowBatchHandle, DataError> {
        self.rag.rag_candidates(collection, query_embedding, top_k).await
    }
}
