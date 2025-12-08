# 51 – Schemas (MVP Canonical Tables)

Arrow-first, v1.0 schemas for the eight shipped tables. Each table is `Spine + Payload`; `row_id` == domain PK.

**Executable spec:** These schemas are implemented by `software/kernel/goni-schema` via the `define_tables!` block in `goni-schema/src/lib.rs`. This document and that DSL must stay in sync.

## Plane 𝒜 – Knowledge (immutable)

### Docs
- PK: `doc_id = row_id`
- Fields: `source_uri: large_utf8`, `mime_type: utf8`, `title: utf8`, `tags: list<utf8>`, `metadata: map<utf8, utf8>`
- Notes: No full text beyond metadata; content lives in Chunks.

### Chunks
- PK: `chunk_id = row_id`
- Fields: `doc_id: fixed_size_binary[16]`, `ordinal: uint32`, `text: large_utf8`, `token_count: uint32`, `section_path: list<utf8>`
- Notes: **Only** raw text column #1 (with Prompts.text in 𝒳).

### Embeddings
- PK: `embedding_id = row_id`
- Fields: `chunk_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `vector: fixed_size_list<float32>[1536]`, `dim: uint16`
- Notes: Lance index on `vector`.

## Plane 𝒳 – Context (ephemeral)

### ContextItems
- PK: `context_item_id = row_id`
- Fields: `context_id: fixed_size_binary[16]`, `chunk_id: fixed_size_binary[16]`, `cost_tokens: uint32`, `selected: bool`, `rank: uint16?`, `marginal_gain: float32?`
- Notes: Submodular selection outputs `selected`/`rank`; joins to 𝒜 via `chunk_id`.

## Plane 𝒦 – Control (metadata only)

### Requests
- PK: `request_id = row_id`
- Fields: `session_id?: fixed_size_binary[16]`, `prompt_hash: fixed_size_binary[32]`, `prompt_tokens_est: uint32`, `budget_tokens: uint32`, `task_class: dict<uint8, utf8>`
- Notes: No raw text; hashes only.

### Tasks
- PK: `task_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `task_type: dict<uint8, utf8>`, `state: dict<uint8, utf8>`, `queue_id: dict<uint8, utf8>`, `expected_cost_tokens: uint32`
- Notes: Lyapunov inputs; append-only state transitions.

## Plane ℰ – Execution (telemetry)

### LlmCalls
- PK: `call_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `prompt_tokens: uint32`, `completion_tokens: uint32`, `total_tokens: uint32`, `latency_ms: uint32`, `cache_hit: bool`
- Notes: Exact billing; may be linked to spans.

### Metrics
- PK: `(name, ts, labels)` or `metric_id = row_id` depending on storage
- Fields: `name: dict<uint8, utf8>`, `value_f64?: float64`, `value_i64?: int64`, `labels: map<utf8, utf8>`
- Notes: Prometheus export compatibility; avoid unbounded label cardinality.

## Plane ?? - Memory (lifecycle-aware)

### MemoryEntries
- PK: `memory_id = row_id`
- Fields: `type: dict<uint8, utf8> (working|episodic|semantic|procedural)`, `timestamp: timestamp(us)`, `importance: float32` (subject to decay), `state: dict<uint8, utf8> (raw|distilled|archived|tombstoned)`, `embedding: fixed_size_list<float32>[1536]?`, `payload: large_utf8`, `links: list<fixed_size_binary[16]>`
- Notes: Lifecycle/retention policy:
  - `working` expires with the session,
  - `episodic` raw distils to semantic after a window (e.g. 30d) then archives,
  - `semantic` persists with decay; can be pinned,
  - `procedural` is versioned and stable.

## Invariants
- All IDs are `fixed_size_binary[16]` (UUIDv7) and equal `Spine.row_id` for their table.
- No `LargeUtf8` outside `Chunks.text` and `Prompts.text` (TXT axiom).
- Dictionaries enumerate finite vocabularies; adding a new label/value requires schema version bump.
