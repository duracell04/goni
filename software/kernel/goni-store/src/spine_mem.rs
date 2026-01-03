use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{ArrowBatchHandle, DataError, DataPlane};

/// In-memory Arrow "spine" data plane.
///
/// Purpose: provide a minimal append-only implementation for Control/Knowledge/Execution tables
/// during early kernel bring-up. This makes specs like AuditRecords/StateSnapshots actionable
/// without committing to DuckDB/LanceDB yet.
///
/// NOTE: This is not durable and does not implement SQL queries.
pub struct InMemorySpineDataPlane {
    tables: Mutex<HashMap<String, Vec<ArrowBatchHandle>>>,
}

impl InMemorySpineDataPlane {
    pub fn new() -> Self {
        Self {
            tables: Mutex::new(HashMap::new()),
        }
    }

    /// Fetch all appended batches for a table (best-effort debug hook).
    pub async fn get_table(&self, table: &str) -> Vec<ArrowBatchHandle> {
        let inner = self.tables.lock().await;
        inner.get(table).cloned().unwrap_or_default()
    }
}

#[async_trait]
impl DataPlane for InMemorySpineDataPlane {
    async fn query(&self, _sql: &str) -> Result<Vec<ArrowBatchHandle>, DataError> {
        Err(DataError {
            message: "InMemorySpineDataPlane does not support SQL queries".into(),
        })
    }

    async fn append_batches(
        &self,
        table: &str,
        batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError> {
        let mut inner = self.tables.lock().await;
        let entry = inner.entry(table.to_string()).or_default();
        for b in batches {
            entry.push(Arc::clone(&b));
        }
        Ok(())
    }

    async fn rag_candidates(
        &self,
        _collection: &str,
        _query_embedding: &[f32],
        _top_k: usize,
    ) -> Result<ArrowBatchHandle, DataError> {
        Err(DataError {
            message: "InMemorySpineDataPlane has no RAG".into(),
        })
    }
}
