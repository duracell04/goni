---
id: GONI-IMAP-9259D3EC104E
title: 3.1 API
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '` ust pub struct CandidateChunk { pub chunk_id: String, pub similarity: f32, pub token_count: u32, pub source: serde_json::Value, } #[async_trait::async_trait] pub trait VecDb { async fn insert_chunks(&self, batch: RecordBatch) -> anyhow::Result<()>; async fn search( &self, query_embedding: &[f32], top_k: usize, ) -> anyhow::Result<RecordBatch>;'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 3.1 API
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 3.1 API

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 API

`
ust
pub struct CandidateChunk {
    pub chunk_id: String,
    pub similarity: f32,
    pub token_count: u32,
    pub source: serde_json::Value,
}

#[async_trait::async_trait]
pub trait VecDb {
    async fn insert_chunks(&self, batch: RecordBatch) -> anyhow::Result<()>;

    async fn search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
    ) -> anyhow::Result<RecordBatch>;

    async fn rebuild(&self) -> anyhow::Result<()>;
}
`
