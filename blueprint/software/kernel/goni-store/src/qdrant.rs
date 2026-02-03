use std::sync::Arc;

use arrow_array::{builder::StringBuilder, types::UInt32Type, Array, ArrayRef, FixedSizeListArray, Float32Array, StringArray, UInt32Array};
use arrow_schema::{DataType, Field, Schema};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use goni_embed::embed;

use crate::{ArrowBatch, ArrowBatchHandle, DataError, DataPlane};

/// Qdrant-backed DataPlane for RAG queries and ingestion.
pub struct QdrantDataPlane {
    client: reqwest::Client,
    base_url: String,
    embed_dim: usize,
}

impl QdrantDataPlane {
    pub fn new(base_url: impl Into<String>) -> Self {
        let embed_dim = std::env::var("EMBED_DIM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1024);
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            embed_dim,
        }
    }

    fn embed(&self, text: &str) -> Vec<f32> {
        embed(text, self.embed_dim)
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
    payload: serde_json::Value,
    #[serde(default)]
    vector: Vec<f32>,
}

#[derive(Serialize)]
struct UpsertPoint<'a> {
    id: &'a str,
    vector: Vec<f32>,
    payload: serde_json::Value,
}

#[derive(Serialize)]
struct UpsertRequest<'a> {
    points: Vec<UpsertPoint<'a>>,
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
        table: &str,
        batches: Vec<ArrowBatchHandle>,
    ) -> Result<(), DataError> {
        for batch in batches {
            let id_idx = batch.schema().index_of("id").map_err(|_| DataError {
                message: "missing id column".into(),
            })?;
            let text_idx = batch.schema().index_of("text").map_err(|_| DataError {
                message: "missing text column".into(),
            })?;
            let tokens_idx = batch
                .schema()
                .index_of("tokens")
                .map_err(|_| DataError {
                    message: "missing tokens column".into(),
                })?;

            let ids = batch.column(id_idx).as_any().downcast_ref::<StringArray>().ok_or(
                DataError {
                    message: "id column not utf8".into(),
                },
            )?;
            let texts = batch.column(text_idx).as_any().downcast_ref::<StringArray>().ok_or(
                DataError {
                    message: "text column not utf8".into(),
                },
            )?;
            let tokens_arr = batch
                .column(tokens_idx)
                .as_any()
                .downcast_ref::<UInt32Array>()
                .ok_or(DataError {
                    message: "tokens column not u32".into(),
                })?;

            let mut points = Vec::with_capacity(batch.num_rows());
            for row in 0..batch.num_rows() {
                if ids.is_null(row) || texts.is_null(row) {
                    continue;
                }
                let id = ids.value(row);
                let text = texts.value(row);
                let tokens = tokens_arr.value(row);
                let vector = self.embed(text);
                let payload = serde_json::json!({
                    "text": text,
                    "tokens": tokens,
                });
                points.push(UpsertPoint {
                    id,
                    vector,
                    payload,
                });
            }

            if points.is_empty() {
                continue;
            }

            let url = format!("{}/collections/{}/points?wait=true", self.base_url, table);
            let body = UpsertRequest { points };
            let resp = self
                .client
                .put(&url)
                .json(&body)
                .send()
                .await
                .map_err(|e| DataError {
                    message: format!("qdrant upsert error: {e}"),
                })?;
            if !resp.status().is_success() {
                return Err(DataError {
                    message: format!("qdrant upsert status: {}", resp.status()),
                });
            }
        }
        Ok(())
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
        let mut id_builder = StringBuilder::new();
        let mut text_builder = StringBuilder::new();
        let mut tokens: Vec<u32> = Vec::with_capacity(parsed.result.len());
        let mut embedding_vals: Vec<f32> = Vec::new();

        let mut dim: Option<usize> = None;
        for item in &parsed.result {
            let id_str = match &item.id {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                _ => "".to_string(),
            };
            id_builder.append_value(&id_str);

            let text_val = item
                .payload
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            text_builder.append_value(text_val);

            let tok_val = item
                .payload
                .get("tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            tokens.push(tok_val);

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
        let text_array = text_builder.finish();
        let token_array: UInt32Array = UInt32Array::from(tokens);
        let value_array: Float32Array = Float32Array::from(embedding_vals);

        let item_field = Arc::new(Field::new("item", DataType::Float32, false));
        let embedding_array = FixedSizeListArray::try_new(
            item_field.clone(),
            dim as i32,
            Arc::new(value_array) as ArrayRef,
            None,
        )
        .map_err(|e| DataError {
            message: format!("embedding array error: {e}"),
        })?;

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("tokens", DataType::UInt32, false),
            Field::new(
                "embedding",
                DataType::FixedSizeList(item_field.clone(), dim as i32),
                false,
            ),
        ]));

        let columns: Vec<ArrayRef> = vec![
            Arc::new(id_array),
            Arc::new(text_array),
            Arc::new(token_array),
            Arc::new(embedding_array),
        ];

        let batch = ArrowBatch::try_new(schema, columns).map_err(|e| DataError {
            message: format!("record batch error: {e}"),
        })?;

        Ok(Arc::new(batch))
    }
}
