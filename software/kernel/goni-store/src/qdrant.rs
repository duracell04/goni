use std::sync::Arc;

use arrow_array::{builder::StringBuilder, types::UInt32Type, ArrayRef, FixedSizeListArray, Float32Array, UInt32Array};
use arrow_schema::{DataType, Field, Schema};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{ArrowBatch, ArrowBatchHandle, DataError, DataPlane};

/// Qdrant-backed DataPlane for RAG queries.
pub struct QdrantDataPlane {
    client: reqwest::Client,
    base_url: String,
}

impl QdrantDataPlane {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }
}

#[derive(Serialize)]
struct SearchRequest<'a> {
    vector: &'a [f32],
    limit: usize,
    with_vector: bool,
    with_payload: bool,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    result: Vec<SearchResult>,
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    #[serde(default)]
    id: serde_json::Value,
    #[serde(default)]
    score: f32,
    #[serde(default)]
    payload: serde_json::Value,
    #[serde(default)]
    vector: Vec<f32>,
}

#[async_trait]
impl DataPlane for QdrantDataPlane {
    async fn query(
        &self,
        _sql: &str,
    ) -> Result<Vec<ArrowBatchHandle>, DataError> {
        Err(DataError {
            message: "QdrantDataPlane does not support SQL queries".into(),
        })
    }

    async fn append_batches(
        &self,
        _table: &str,
        _batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError> {
        Err(DataError {
            message: "append_batches not implemented for QdrantDataPlane".into(),
        })
    }

    async fn rag_candidates(
        &self,
        collection: &str,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<ArrowBatchHandle, DataError> {
        let url = format!("{}/collections/{}/points/search", self.base_url, collection);
        let body = SearchRequest {
            vector: query_embedding,
            limit: top_k,
            with_vector: true,
            with_payload: true,
        };

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| DataError {
                message: format!("qdrant request error: {e}"),
            })?;

        if !resp.status().is_success() {
            return Err(DataError {
                message: format!("qdrant status: {}", resp.status()),
            });
        }

        let parsed: SearchResponse = resp.json().await.map_err(|e| DataError {
            message: format!("qdrant parse error: {e}"),
        })?;

        // Collect fields
        let mut id_builder = StringBuilder::new(parsed.result.len());
        let mut tokens: Vec<u32> = Vec::with_capacity(parsed.result.len());
        let mut embedding_vals: Vec<f32> = Vec::new();

        let mut dim: Option<usize> = None;
        for item in &parsed.result {
            // id -> string
            let id_str = match &item.id {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                _ => "".to_string(),
            };
            id_builder.append_value(id_str).map_err(|e| DataError {
                message: format!("id build error: {e}"),
            })?;

            // tokens from payload if present
            let tok_val = item
                .payload
                .get("tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            tokens.push(tok_val);

            // embedding vector
            if let Some(d) = dim {
                if item.vector.len() != d {
                    return Err(DataError {
                        message: "embedding dimension mismatch across results".into(),
                    });
                }
            } else {
                dim = Some(item.vector.len());
            }
            embedding_vals.extend_from_slice(&item.vector);
        }

        let dim = dim.unwrap_or(0);
        if dim == 0 {
            return Err(DataError {
                message: "qdrant returned no embeddings".into(),
            });
        }

        let id_array = id_builder.finish();
        let token_array: UInt32Array = UInt32Array::from(tokens);
        let value_array: Float32Array = Float32Array::from(embedding_vals);

        let embedding_array = FixedSizeListArray::try_new(
            value_array.into(),
            dim as i32,
        )
        .map_err(|e| DataError {
            message: format!("embedding array error: {e}"),
        })?;

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("tokens", DataType::UInt32, false),
            Field::new(
                "embedding",
                DataType::FixedSizeList(
                    Box::new(Field::new("item", DataType::Float32, false)),
                    dim as i32,
                ),
                false,
            ),
        ]));

        let columns: Vec<ArrayRef> = vec![
            Arc::new(id_array),
            Arc::new(token_array),
            Arc::new(embedding_array),
        ];

        let batch = ArrowBatch::try_new(schema, columns).map_err(|e| DataError {
            message: format!("record batch error: {e}"),
        })?;

        Ok(Arc::new(batch))
    }
}
