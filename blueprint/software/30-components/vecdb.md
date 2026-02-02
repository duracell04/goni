# 70 – Vector Database

Status: MVP – single-node ANN index + Arrow adapter  
Scope: Local RAG retrieval for Context Plane

---

## 1. Role in the system

The **Vector Database** (VecDB) is the retrieval backend for the Context Plane (??):

- Stores embeddings and metadata for chunks,
- Performs approximate nearest neighbour (ANN) search,
- Returns candidates as Arrow RecordBatches on the Arrow Spine (??).

It is a concrete implementation of the retrieval part feeding the submodular context selector.

Note: retrieval is treated as a Memory Plane capability invoked by the
predictor as evidence. It augments latent state; it is not the cognitive core.
See `blueprint/docs/specs/latent-state-contract.md` and
`blueprint/docs/specs/tool-capability-api.md`.

---

## 2. Responsibilities & boundaries

### 2.1 Responsibilities

- **Storage & indexing**
  - Maintain an index over (chunk_id, embedding, token_count, source_metadata).

- **ANN search**
  - Given a query embedding q, return top-K nearest neighbours with similarity scores.

- **Arrow integration**
  - Expose results as RecordBatch with a fixed schema usable by ??.

### 2.2 Non-responsibilities

- ? Splitting raw documents into chunks (ingestion pipeline).  
- ? Computing embeddings (embedding runtime).  
- ? Choosing which of the retrieved candidates go into the final prompt (selector).

---

## 3. Interfaces

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

### 3.2 Result schema (Arrow)

Search results MUST at least expose:

* chunk_id: Utf8,
* similarity: Float32,
* 	oken_count: UInt32,
* source_meta: LargeBinary or similar.

This schema is what goni-context expects for the submodular objective and budget accounting.

---

## 4. Invariants & MVP targets

* **Arrow Spine invariant**
  search returns an Arrow RecordBatch wired into ??; no JSON/serde in the hot path.

* **Quality target (recall)**
  For small synthetic benchmarks, ANN recall@K vs brute-force = 0.9.

* **Latency target**
  For typical K (e.g. 64–128) and dataset sizes expected for a single user, p99 search latency « LLM latency (target < 50 ms).

* **Freshness**
  New chunks should be searchable “soon enough” (MVP: after a bounded delay, or on explicit 
ebuild).

---

## 5. MVP vs future VecDB

**MVP**

* Single-node index with modest document set.
* One backend (e.g. DuckDB+Lance, Qdrant, etc.) is sufficient.

**Future**

* Sharded / partitioned indices across nodes in the mesh.
* Hybrid lexical + vector retrieval.
* Rich filtering (by source, time, tags) at VecDB level.



## 6. Upstream
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Privacy and text confinement](/blueprint/software/50-data/40-privacy-and-text-confinement.md)

## 7. Downstream
- [Latent predictor](/blueprint/software/30-components/latent-predictor.md)

## 8. Adjacent
- [Tool capability API](/blueprint/software/docs/specs/tool-capability-api.md)
- [Orchestrator](/blueprint/software/30-components/orchestrator.md)
