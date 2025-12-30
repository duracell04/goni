use async_trait::async_trait;
use arrow::array::{
    Array, Float32Array, FixedSizeListArray, Int32Array, StringArray, UInt32Array,
};
use arrow::datatypes::DataType;
use arrow::record_batch::RecordBatch;
use goni_types::{ContextSelection, KvPageId};
use thiserror::Error;

/// Minimal view of a candidate chunk for context selection.
#[derive(Clone)]
pub struct CandidateChunk<'a> {
    pub id: &'a str,
    pub text: Option<&'a str>,
    pub tokens: usize,
    pub embedding: &'a [f32],
    pub relevance: f32,
}

/// Semantic selector – implements the submodular context optimization.
#[async_trait]
pub trait ContextSelector: Send + Sync {
    async fn select<'a>(
        &self,
        query_embedding: &[f32],
        candidates: &[CandidateChunk<'a>],
        max_tokens: usize,
    ) -> ContextSelection;
}

/// KV cache pager – manages on-device KV pages.
#[async_trait]
pub trait KvPager: Send + Sync {
    /// Ensure these pages are resident on device (GPU/NPU).
    async fn ensure_resident(
        &self,
        pages: &[KvPageId],
    ) -> Result<(), KvError>;

    /// Notify about newly created KV pages and receive eviction candidates.
    async fn on_new_pages(
        &self,
        new_pages: &[KvPageId],
    ) -> Result<Vec<KvPageId>, KvError>;

    /// Report which pages were actually touched in last forward pass.
    async fn report_access(
        &self,
        accessed: &[KvPageId],
    ) -> Result<(), KvError>;
}

#[derive(Debug)]
pub struct KvError {
    pub message: String,
}

//
// 1) FacilityLocationSelector – real submodular context selector
//

/// Facility-location based context selector.
///
/// Objective:
///   F(S) = Σ_i max_{j∈S} cos(e_i, e_j) + γ Σ_{j∈S} relevance_j
///
/// Greedy algorithm gives a (1 - 1/e)-approximation to the optimal S.
pub struct FacilityLocationSelector {
    /// Weight on the relevance term (γ).
    gamma: f32,
}

impl FacilityLocationSelector {
    pub fn new(gamma: f32) -> Self {
        Self { gamma }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }
}

#[async_trait]
impl ContextSelector for FacilityLocationSelector {
    async fn select<'a>(
        &self,
        _query_embedding: &[f32],
        candidates: &[CandidateChunk<'a>],
        max_tokens: usize,
    ) -> ContextSelection {
        let n = candidates.len();
        if n == 0 || max_tokens == 0 {
            return ContextSelection {
                indices: Vec::new(),
                total_tokens: 0,
            };
        }

        // 1) Precompute similarity matrix sim[i][j] = cos(e_i, e_j)
        let mut sim: Vec<f32> = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                let s = Self::cosine_similarity(
                    candidates[i].embedding,
                    candidates[j].embedding,
                );
                sim[i * n + j] = s.max(0.0); // clamp to non-negative
            }
        }

        // 2) Greedy selection
        let mut selected_indices: Vec<u32> = Vec::new();
        let mut selected_mask = vec![false; n];
        let mut coverage: Vec<f32> = vec![0.0; n]; // cov[i] = max_{j∈S} sim(i,j)
        let mut remaining_tokens = max_tokens;

        loop {
            let mut best_gain = 0.0_f32;
            let mut best_idx: Option<usize> = None;

            for j in 0..n {
                if selected_mask[j] {
                    continue;
                }

                let tok = candidates[j].tokens;
                if tok > remaining_tokens {
                    continue; // can't fit
                }

                // Compute marginal gain of adding j:
                // ΔF = Σ_i (max(cov[i], sim(i,j)) - cov[i]) + γ * relevance_j
                let mut gain_cov = 0.0_f32;
                for i in 0..n {
                    let s = sim[i * n + j];
                    let new_cov = if s > coverage[i] { s } else { coverage[i] };
                    gain_cov += new_cov - coverage[i];
                }
                let gain = gain_cov + self.gamma * candidates[j].relevance;

                if gain > best_gain {
                    best_gain = gain;
                    best_idx = Some(j);
                }
            }

            match best_idx {
                Some(j) if best_gain > 0.0 => {
                    // Select j
                    selected_mask[j] = true;
                    selected_indices.push(j as u32);
                    remaining_tokens =
                        remaining_tokens.saturating_sub(candidates[j].tokens);

                    // Update coverage array
                    for i in 0..n {
                        let s = sim[i * n + j];
                        if s > coverage[i] {
                            coverage[i] = s;
                        }
                    }

                    // If we run out of tokens, stop.
                    if remaining_tokens == 0 {
                        break;
                    }
                }
                _ => {
                    // No candidate yields positive marginal gain or fits in budget.
                    break;
                }
            }
        }

        let total_tokens: usize = selected_indices
            .iter()
            .map(|&idx| candidates[idx as usize].tokens)
            .sum();

        ContextSelection {
            indices: selected_indices,
            total_tokens,
        }
    }
}

//
// 1b) Zero-copy converter from RecordBatch → CandidateChunk<'a>
//

#[derive(Debug, Error)]
pub enum CandidateBuildError {
    #[error("missing column '{0}'")]
    MissingColumn(String),
    #[error("invalid type for column '{0}'")]
    InvalidColumnType(String),
    #[error("embedding dimension mismatch; expected {expected}, got {actual}")]
    EmbeddingDimMismatch { expected: usize, actual: usize },
}

/// Cosine similarity used for relevance.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

/// Convert an Arrow RecordBatch into zero-copy CandidateChunk<'a> values.
///
/// Assumes schema:
///   - id_col: Utf8
///   - tokens_col: Int32 or UInt32
///   - embedding_col: FixedSizeList<Float32> with length == query_embedding.len()
///   - text_col: Utf8 (optional; set None if not present)
///
/// IMPORTANT:
///   - The returned CandidateChunk<'a> borrows from `batch`, so `batch` must
///     outlive the returned Vec.
///   - No copies of embeddings or ids are performed: all slices &str/&[f32]
///     point into Arrow's internal buffers.
pub fn record_batch_to_candidate_chunks<'a>(
    batch: &'a RecordBatch,
    id_col: &str,
    tokens_col: &str,
    embedding_col: &str,
    query_embedding: &[f32],
) -> Result<Vec<CandidateChunk<'a>>, CandidateBuildError> {
    let schema = batch.schema();

    // 1) Locate columns by name
    let id_idx = schema
        .index_of(id_col)
        .map_err(|_| CandidateBuildError::MissingColumn(id_col.to_string()))?;
    let tokens_idx = schema.index_of(tokens_col).map_err(|_| {
        CandidateBuildError::MissingColumn(tokens_col.to_string())
    })?;
    let emb_idx = schema.index_of(embedding_col).map_err(|_| {
        CandidateBuildError::MissingColumn(embedding_col.to_string())
    })?;
    let text_idx = schema.index_of("text").ok();

    // 2) Downcast columns

    // id: Utf8
    let id_array = batch.column(id_idx);
    let id_array = id_array.as_any().downcast_ref::<StringArray>().ok_or_else(
        || CandidateBuildError::InvalidColumnType(id_col.to_string()),
    )?;

    // optional text
    let text_array: Option<&StringArray> = match text_idx {
        Some(idx) => batch.column(idx).as_any().downcast_ref::<StringArray>(),
        None => None,
    };

    // tokens: Int32 or UInt32 → usize
    let tokens_array = batch.column(tokens_idx);
    let tokens_type = tokens_array.data_type();

    // embedding: FixedSizeList<Float32>
    let emb_array = batch.column(emb_idx);
    let emb_list = emb_array.as_any().downcast_ref::<FixedSizeListArray>().ok_or_else(
        || CandidateBuildError::InvalidColumnType(embedding_col.to_string()),
    )?;

    let value_len = emb_list.value_length() as usize;
    let emb_values = emb_list.values();
    let emb_values = emb_values.as_any().downcast_ref::<Float32Array>().ok_or_else(
        || CandidateBuildError::InvalidColumnType(embedding_col.to_string()),
    )?;

    if value_len != query_embedding.len() {
        return Err(CandidateBuildError::EmbeddingDimMismatch {
            expected: query_embedding.len(),
            actual: value_len,
        });
    }

    // This is the contiguous f32 buffer for ALL embeddings.
    let raw_buf = emb_values.values();
    // Arrow 54: ScalarBuffer exposes a slice view for typed data.
    let raw_slice: &[f32] = raw_buf.as_ref();

    let num_rows = batch.num_rows();
    let mut chunks = Vec::with_capacity(num_rows);

    for row in 0..num_rows {
        // 3) id as &str (zero-copy)
        if id_array.is_null(row) {
            continue;
        }
        let id_str: &str = id_array.value(row);

        let text_val = text_array.and_then(|arr| if arr.is_null(row) { None } else { Some(arr.value(row)) });

        // 4) tokens as usize
        let tokens: usize = match tokens_type {
            DataType::Int32 => {
                let ints = tokens_array.as_any().downcast_ref::<Int32Array>().ok_or_else(
                    || CandidateBuildError::InvalidColumnType(tokens_col.to_string()),
                )?;
                let v = ints.value(row);
                if v <= 0 {
                    continue;
                }
                v as usize
            }
            DataType::UInt32 => {
                let ints = tokens_array.as_any().downcast_ref::<UInt32Array>().ok_or_else(
                    || CandidateBuildError::InvalidColumnType(tokens_col.to_string()),
                )?;
                ints.value(row) as usize
            }
            _ => {
                return Err(CandidateBuildError::InvalidColumnType(
                    tokens_col.to_string(),
                ))
            }
        };

        // 5) embedding slice for this row:
        // FixedSizeListArray layout packs all rows contiguously.
        let start = row * value_len;
        let end = start + value_len;
        let emb_slice: &[f32] = &raw_slice[start..end];

        // 6) relevance = cos(query_embedding, embedding)
        let relevance = cosine_similarity(query_embedding, emb_slice);

        chunks.push(CandidateChunk {
            id: id_str,
            text: text_val,
            tokens,
            embedding: emb_slice,
            relevance,
        });
    }

    Ok(chunks)
}

//
// 2) NullKvPager – simple no-op pager (fine as-is)
//

pub struct NullKvPager;

#[async_trait]
impl KvPager for NullKvPager {
    async fn ensure_resident(
        &self,
        _pages: &[KvPageId],
    ) -> Result<(), KvError> {
        Ok(())
    }

    async fn on_new_pages(
        &self,
        _new_pages: &[KvPageId],
    ) -> Result<Vec<KvPageId>, KvError> {
        Ok(Vec::new())
    }

    async fn report_access(
        &self,
        _accessed: &[KvPageId],
    ) -> Result<(), KvError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn selector_respects_budget_and_is_deterministic() {
        let selector = FacilityLocationSelector::new(0.1);
        let query = vec![1.0_f32, 0.0];

        let candidates = vec![
            CandidateChunk {
                id: "a",
                text: Some("foo"),
                tokens: 3,
                embedding: &[1.0, 0.0],
                relevance: 0.9,
            },
            CandidateChunk {
                id: "b",
                text: Some("bar"),
                tokens: 2,
                embedding: &[0.0, 1.0],
                relevance: 0.8,
            },
            CandidateChunk {
                id: "c",
                text: None,
                tokens: 10,
                embedding: &[0.7, 0.7],
                relevance: 0.5,
            },
        ];

        let sel1 = selector.select(&query, &candidates, 4).await;
        let sel2 = selector.select(&query, &candidates, 4).await;

        assert!(sel1.total_tokens <= 4);
        assert_eq!(sel1.indices, sel2.indices, "deterministic selection");
        assert!(!sel1.indices.is_empty(), "should select at least one chunk");
    }
}
