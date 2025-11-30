# 50 – Data Ontology & Schema Invariants (Arrow Spine)  
**v1.0 – Frozen Specification (2027-04-01)**  
**Status:** Normative. This document is the single source of truth for every table, column, and ID in Goni v1.0.

## 0. The Three Axioms (non-negotiable, CI-enforced)

| Axiom | Statement | Enforcement |
|------|-----------|-------------|
| SMA  | Every persistent or transient entity is exactly one row in one of the tables below. | `cargo deny` + custom Clippy lint + schema registry |
| ZCO  | No crate boundary may exchange a non-Arrow domain type for a canonical entity. Only `RecordBatch` or `Arc<RecordBatch>` + opaque IDs. | `forbid(non_arrow_entity)` lint + review gate |
| TXT  | Raw natural-language text (`LargeUtf8` > 512 bytes) exists in exactly two columns system-wide: `Chunks.text` and `Prompts.text`. | Schema validator rejects any new `LargeUtf8` column |

Violation of any axiom is a compile-time error in CI.

## 1. The Four Planes – Final Partitioning

| Plane | Symbol | Lifetime | Sensitivity | Storage | Primary Keys |
|-------|--------|----------|-------------|---------|--------------|
| Knowledge | 𝒜 | Permanent | High (raw text) | Parquet + Lance v2 | `doc_id`, `chunk_id`, `embedding_id` |
| Context | 𝒳 | ≤ 24 h | High (live prompts) | Memory + optional spill to encrypted tmp | `request_id`, `context_id` |
| Control | 𝒦 | Permanent | Low (metadata only) | WAL + Parquet | `request_id`, `task_id` |
| Execution | ℰ | Permanent | Low (aggregates) | Parquet (append-only) | `span_id`, `call_id` |

## 2. The Universal Spine (exact Arrow schema)

```arrow
struct Spine {
  row_id          : fixed_size_binary[16]   // UUIDv7 – globally unique, monotonic
  tenant_id       : fixed_size_binary[16]   // single-node: fixed 000…001
  plane           : uint8                   // 0=𝒜 1=𝒳 2=𝒦 3=ℰ
  kind            : dictionary<uint8, utf8> // table name for debugging
  schema_version  : uint16                  // bump on breaking change
  ts_created      : timestamp[ms, UTC]
  ts_valid_from   : timestamp[ms, UTC]      // for SCD-2 facts
}
```

Every table is `Spine + Payload`. No exceptions.

## 3. Canonical Tables – Final v1.0 Schemas

### Plane 𝒜 – Knowledge (immutable corpus)

| Table       | PK             | Payload columns (beyond Spine)                                                                 | Notes |
|-------------|----------------|-----------------------------------------------------------------------------------------------|-------|
| Docs        | doc_id = row_id| source_uri: large_utf8, mime_type: utf8, title: utf8, tags: list<utf8>, metadata: map<utf8,utf8> | No full text |
| Chunks      | chunk_id       | doc_id, ordinal: uint32, text: large_utf8, token_count: uint32, section_path: list<utf8>       | **Only** raw text column #1 |
| Embeddings  | embedding_id   | chunk_id, model_id: dict, vector: fixed_size_list<f32>[1536], dim: uint16                   | Lance index on vector |

### Plane 𝒳 – Live Context (ephemeral)

| Table       | PK             | Payload columns (beyond Spine)                                                                 | Lifetime |
|-------------|----------------|-----------------------------------------------------------------------------------------------|----------|
| Prompts     | prompt_id      | request_id, role: dict, text: large_utf8, token_count: uint32                                 | ≤ 7 d   |
| Contexts    | context_id     | request_id, router_decision_id?, budget_tokens: uint32                                         | ≤ 24 h  |
| ContextItems| item_id        | context_id, chunk_id, cost_tokens: uint32, selected: bool, rank: uint16?, marginal_gain: f32? | ≤ 24 h  |

### Plane 𝒦 – Control (metadata only)

| Table            | PK             | Payload columns (beyond Spine)                                                                 | Notes |
|------------------|----------------|-----------------------------------------------------------------------------------------------|-------|
| Requests         | request_id     | session_id?, prompt_hash: fixed_size_binary[32], prompt_tokens_est: uint32, budget_tokens: uint32, task_class: dict | No raw text |
| Tasks            | task_id        | request_id, task_type: dict, state: dict, queue_id: dict, expected_cost_tokens: uint32       | Lyapunov input |
| QueueSnapshot    | (queue_id, ts)| backlog_tasks: uint64, backlog_tokens: uint64, service_rate_tps: f32                          | 100 ms cadence |
| RouterDecisions   | decision_id    | request_id, policy_version: uint16, chosen_model: dict, features: map<utf8,f32>, voe: f32    | Regret analysis |

### Plane ℰ – Execution & Metrics (telemetry)

| Table     | PK           | Payload columns (beyond Spine)                                                                 | Notes |
|-----------|--------------|-----------------------------------------------------------------------------------------------|-------|
| Spans     | span_id      | parent_span_id?, name: utf8, start/end: timestamp[ms], attrs: map<utf8,utf8>                 | OpenTelemetry compatible |
| LlmCalls  | call_id      | request_id, model_id: dict, prompt/comp/total_tokens: uint32, latency_ms: uint32, cache_hit: bool | Exact billing |
| Metrics   | (name, ts, labels) | value_f64 or value_i64, labels: map<utf8,utf8>                                          | Prometheus export |

## 4. Privacy Theorem (formally provable)

**Theorem (Text Confinement)**  
The only two columns in the entire system with `LargeUtf8` and average length > 512 bytes are:
- `Chunks.text`
- `Prompts.text`

All other columns are bounded ≤ 256 bytes or numeric/enum.

**Corollary**  
Any export of the union of planes 𝒦 ∪ ℰ is automatically safe for analytics, sharing, or off-device upload (no PII leakage possible by schema).

## 5. Zero-Copy Algorithm Mapping (exact)

| Algorithm                | Input tables                         | Output tables / columns mutated          | Copies |
|--------------------------|-------------------------------------|----------------------------------------|-------|
| Submodular selection      | Embeddings + Chunks + ContextItems   | ContextItems.selected, .rank            | 0     |
| Lyapunov scheduling        | QueueSnapshot + Tasks                 | Tasks.state, QueueSnapshot                | 0     |
| Router regret             | RouterDecisions + LlmCalls + Metrics | RouterDecisions.offline_reward_estimate    | 0     |
| KV paging                | ContextItems + internal page table    | page residency flags                     | 0     |

All four core theorems from the theory doc operate on Arrow arrays directly.

## 6. Schema Evolution Policy (CI gate)

1. Never delete or rename columns.
2. New columns added with default `null`.
3. `schema_version` bump on any semantic change.
4. Old binaries must read new files (forward compatibility enforced by Arrow reader).

## 7. MVP Table Set (what actually ships in v1.0 binary)

We ship **exactly eight** tables:

1. Docs  
2. Chunks  
3. Embeddings  
4. Requests  
5. Tasks  
6. ContextItems  
7. LlmCalls  
8. Metrics  

The remaining tables (Prompts, QueueSnapshot, RouterDecisions, Spans) are derived or optional in v1.0 and may be materialised later without breaking anything.

## 8. Enforcement in Code

```toml
# workspace Cargo.toml
[workspace.metadata.schema]
tables = [
  "Docs", "Chunks", "Embeddings",
  "Requests", "Tasks", "ContextItems",
  "LlmCalls", "Metrics"
]
forbidden_types = ["LargeUtf8"]   # except in Chunks.text and Prompts.text
```

A custom Cargo checker rejects any new struct containing these entities outside the schema registry.

This is the **final, locked, mathematically clean, zero-copy, privacy-preserving data ontology** for Goni v1.0.

No deviations allowed.  
All future features must extend these tables only.  

We now implement against it.

## Goni Data Ontology: Arrow-Native Schema Design

### Core Invariants

**1. Arrow-First Representation**
- All persistent state manifests as Arrow Tables/RecordBatches
- Disk: Partitioned Parquet datasets; Memory: Zero-copy mmap buffers
- Schema evolution via additive columns with semantic versioning

**2. Referential Integrity**
- Opaque 128-bit identifiers establish cross-plane relationships
- All foreign keys reference spine `id` fields
- Joins expressible as Arrow compute operations

**3. Privacy by Construction**
- Raw text confinement to planes 𝒜 (Knowledge) and 𝒳 (Context)
- Control plane 𝒦 and execution plane 𝓔 operate exclusively on hashes, IDs, and aggregates
- Schema-level guarantees: 𝓔 and 𝒦 tables contain no `large_utf8` columns

### Formal Schema Definitions

#### Shared Spine (All Tables)
```arrow
struct Spine {
  // Primary identifier (UUIDv7 for temporal locality)
  id: fixed_size_binary(16)
  
  // Multi-tenant isolation
  tenant_id: fixed_size_binary(16)
  
  // Plane classification (0=𝒜, 1=𝒳, 2=𝒦, 3=𝓔)  
  plane: uint8
  
  // Table-type discriminant (dictionary-encoded)
  kind: dictionary<uint8, string>
  
  // Semantic versioning for schema evolution
  schema_version: uint16
  
  // Creation timestamp (microsecond precision, UTC)
  created_at: timestamp[ms, tz=UTC]
}
```

#### Plane 𝒜: Knowledge Base (Immutable Corpus)

**Docs** - Logical source documents
```arrow
struct Docs {
  // Spine columns
  spine: Spine
  
  // Explicit primary key (redundant with spine.id but explicit)
  doc_id: fixed_size_binary(16)  // = spine.id
  
  // Content classification
  source_type: dictionary<uint8, string>  // "file" | "email" | "web_page"
  external_uri: string                     // Content-addressable storage key
  mime_type: string
  
  // Descriptive metadata (limited-length text only)
  title: string                           // Max 1024 chars
  created_at_source: timestamp[ms, tz=UTC]?
  
  // Organizational taxonomy
  tags: list<string>                      // String array for categorization
  metadata: map<string, string>           // Connector-extracted key-values
}
```

**Chunks** - Atomic retrieval units for RAG
```arrow
struct Chunks {
  spine: Spine
  chunk_id: fixed_size_binary(16)        // = spine.id
  
  // Document relationship
  doc_id: fixed_size_binary(16)          // Foreign key to Docs
  
  // Structural positioning  
  ordinal: int32                         // Intra-document ordering
  section_path: list<string>             // Hierarchical document structure
  
  // Content (raw text - high privacy)
  text: large_string                     // Confined raw text storage
  
  // Precomputed metrics for cost estimation
  token_count: int32
  char_count: int32
  
  // Lifecycle management
  is_deleted: bool                       // Logical deletion marker
  version: uint32                        // Content version for updates
}
```

**Embeddings** - Vector representations
```arrow
struct Embeddings {
  spine: Spine
  embedding_id: fixed_size_binary(16)    // = spine.id
  
  // Chunk relationship
  chunk_id: fixed_size_binary(16)        // Foreign key to Chunks
  
  // Model specification
  model_id: dictionary<uint8, string>    // "text-embedding-3-small", etc.
  model_version: string                  // Full semantic version
  
  // Vector data
  vector: fixed_size_list<float32>(d)    // d-dimensional embedding
  dim: int16                             // Dimension cardinality d
  
  // Generation metadata
  generated_at: timestamp[ms, tz=UTC]
  generation_duration_ms: int32
}
```

#### Plane 𝒦: Control System (Metadata Only)

**Requests** - User interaction units
```arrow
struct Requests {
  spine: Spine
  request_id: fixed_size_binary(16)      // = spine.id
  
  // Session context
  session_id: fixed_size_binary(16)?
  user_id: fixed_size_binary(16)?        // Device-scoped pseudonym
  
  // API specification
  api_endpoint: dictionary<uint8, string>  // "/v1/chat/completions", etc.
  task_class: dictionary<uint8, string>    // "qa" | "summarize" | "agent"
  
  // Prompt metadata (no raw text)
  prompt_hash: fixed_size_binary(32)     // SHA-256 of raw prompt
  prompt_length_chars: int32
  prompt_tokens_estimate: int32
  
  // Execution parameters
  max_tokens: int32
  temperature: float32
  priority: int16                        // SLA priority class (0-255)
  budget_tokens: int32                   // Total token allocation
  
  // Timing
  deadline_at: timestamp[ms, tz=UTC]?
}
```

**Tasks** - Scheduling units for Lyapunov control
```arrow
struct Tasks {
  spine: Spine
  task_id: fixed_size_binary(16)         // = spine.id
  
  // Request relationship
  request_id: fixed_size_binary(16)      // Foreign key to Requests
  parent_task_id: fixed_size_binary(16)? // For task DAGs
  
  // Task specification
  task_type: dictionary<uint8, string>   // "embed" | "rerank" | "llm_call"
  state: dictionary<uint8, string>       // FSM: "queued"|"running"|"done"|"failed"
  queue_id: dictionary<uint8, string>    // Logical queue assignment
  
  // Resource estimation
  expected_cost_tokens: int32
  expected_duration_ms: int32
  
  // Scheduling constraints
  deadline_ms: int64                     // Epoch milliseconds
  priority: int16                        // Intra-queue priority
  
  // Execution timeline (denormalized for filtering)
  created_at: timestamp[ms, tz=UTC]      // = spine.created_at
  started_at: timestamp[ms, tz=UTC]?
  finished_at: timestamp[ms, tz=UTC]?
  
  // Completion status
  error_code: dictionary<uint8, string>? // If failed
  result_hash: fixed_size_binary(32)?    // Deterministic output hash
}
```

**QueueState** - Lyapunov control state
```arrow
struct QueueState {
  spine: Spine
  
  // Queue identification  
  queue_id: dictionary<uint8, string>    // Logical queue name
  
  // Backlog metrics
  backlog_tokens: int64
  backlog_tasks: int64
  
  // Performance characteristics
  service_rate_tokens_per_s: float32
  avg_task_duration_ms: float32
  
  // Control parameters
  priority_weight: float32
  capacity_tokens_per_s: float32
  
  // Snapshot timestamp
  measured_at: timestamp[ms, tz=UTC]
}
```

**RouterDecisions** - Bandit policy decisions
```arrow
struct RouterDecisions {
  spine: Spine
  decision_id: fixed_size_binary(16)     // = spine.id
  
  // Request context
  request_id: fixed_size_binary(16)      // Foreign key to Requests
  
  // Policy specification
  policy_id: dictionary<uint8, string>   // Versioned router policy
  policy_version: string
  
  // Selection outcome
  chosen_model_id: dictionary<uint8, string>      // "gpt-4.1-mini", etc.
  candidate_model_ids: list<dictionary<uint8, string>>  // Considered alternatives
  
  // Feature vector (privacy-safe numeric features only)
  features: map<string, float32>         // prompt_length, hour_of_day, etc.
  
  // Cost and exploration
  estimated_cost_tokens: int32
  exploration_probability: float32       // For Thompson sampling
  
  // Counterfactual analysis (filled post-hoc)
  offline_reward_estimate: float32?
  counterfactual_model_ids: list<dictionary<uint8, string>>?
}
```

#### Plane 𝒳: Execution Context (Ephemeral/High-Privacy)

**Prompts** - Raw prompt text storage
```arrow
struct Prompts {
  spine: Spine
  prompt_id: fixed_size_binary(16)       // = spine.id
  
  // Request relationship
  request_id: fixed_size_binary(16)      // Foreign key to Requests
  
  // Message role  
  role: dictionary<uint8, string>        // "system" | "user" | "assistant" | "tool"
  
  // Raw text content (high privacy)
  text: large_string
  
  // Token metrics
  token_count: int32
  
  // Privacy handling
  is_redacted: bool                      // PII removal indicator
  redaction_version: uint16?
}
```

**Contexts** - LLM call context bundles
```arrow
struct Contexts {
  spine: Spine
  context_id: fixed_size_binary(16)      // = spine.id
  
  // Request and routing context
  request_id: fixed_size_binary(16)      // Foreign key to Requests
  router_decision_id: fixed_size_binary(16)?  // FK to RouterDecisions
  
  // Prompt reference
  prompt_id: fixed_size_binary(16)       // Foreign key to Prompts
  
  // Resource allocation
  budget_tokens: int32                   // Context window allocation
  used_tokens_estimate: int32            // Post-selection estimate
  
  // Generation parameters
  temperature: float32?
  max_tokens: int32?
}
```

**ContextItems** - Submodular selection working set
```arrow
struct ContextItems {
  spine: Spine
  context_item_id: fixed_size_binary(16) // = spine.id
  
  // Context relationship
  context_id: fixed_size_binary(16)      // Foreign key to Contexts
  
  // Chunk reference (zero-copy)
  chunk_id: fixed_size_binary(16)        // Foreign key to Chunks
  
  // Selection features
  marginal_gain: float32?                // Submodular utility score
  cost_tokens: int32                     // Token cost if selected
  
  // Diversity constraints
  group_key: dictionary<uint8, string>?  // Diversity constraint key
  
  // Selection outcome
  selected: bool                         // Final selection indicator
  rank: int32?                           // Selection ordering
  
  // Algorithmic metadata
  selection_algorithm: dictionary<uint8, string>?  // "greedy" | "lazy-greedy"
  selection_version: uint16?
}
```

#### Plane 𝓔: Execution Telemetry (Append-Only/Low-Privacy)

**Spans** - Distributed tracing spans
```arrow
struct Spans {
  spine: Spine
  span_id: fixed_size_binary(16)         // = spine.id
  
  // Trace topology
  parent_span_id: fixed_size_binary(16)?
  trace_id: fixed_size_binary(16)
  
  // Execution context
  request_id: fixed_size_binary(16)?     // Optional for background jobs
  task_id: fixed_size_binary(16)?
  
  // Span specification
  name: string                           // Span name (limited cardinality)
  kind: dictionary<uint8, string>        // "internal" | "server" | "client"
  
  // Timing
  start_at: timestamp[ms, tz=UTC]
  end_at: timestamp[ms, tz=UTC]
  duration_ms: int32
  
  // Attributes (no PII - dictionary encoded where possible)
  attributes: map<string, string>        // model_id, error codes, etc.
  status: dictionary<uint8, string>      // "ok" | "error" | "timeout"
}
```

**LlmCalls** - Specialized LLM execution spans
```arrow
struct LlmCalls {
  spine: Spine
  call_id: fixed_size_binary(16)         // = spine.id
  
  // Span relationship
  span_id: fixed_size_binary(16)         // Foreign key to Spans
  
  // Request and context
  request_id: fixed_size_binary(16)      // Foreign key to Requests
  context_id: fixed_size_binary(16)      // Foreign key to Contexts
  
  // Model specification
  model_id: dictionary<uint8, string>
  model_version: string?
  
  // Token usage (from provider)
  prompt_tokens: int32
  completion_tokens: int32
  total_tokens: int32
  
  // Performance metrics
  latency_ms: int32
  time_to_first_token_ms: int32?
  
  // Caching and status
  cache_hit: bool
  status: dictionary<uint8, string>      // "ok" | "timeout" | "rate_limited"
  
  // Cost tracking
  estimated_cost_usd: float32?
}
```

**Metrics** - Aggregated performance metrics
```arrow
struct Metrics {
  spine: Spine
  metric_id: fixed_size_binary(16)       // = spine.id
  
  // Execution context (optional)
  request_id: fixed_size_binary(16)?
  call_id: fixed_size_binary(16)?
  task_id: fixed_size_binary(16)?
  
  // Metric specification
  name: dictionary<uint8, string>        // "latency_ms" | "tokens_total" | "cache_hit_rate"
  
  // Value (typed union)
  value_float: float64?
  value_int: int64?
  
  // Dimensional labels (for aggregation)
  labels: map<string, string>            // "task_class" -> "qa", "model_id" -> "gpt-4", etc.
  
  // Temporal aggregation
  bucket_start: timestamp[ms, tz=UTC]?   // For pre-aggregated metrics
  bucket_duration_ms: int32?
}
```

### Privacy Enforcement Theorem

**Textual Confinement Invariant**: 
For any table T in planes 𝓔 or 𝒦, the schema S_T contains no columns of type `large_string` or variable-length binary types.

**Proof**: 
By construction, the union of schemas for 𝓔 and 𝒦 tables (Requests, Tasks, QueueState, RouterDecisions, Spans, LlmCalls, Metrics) contains only:
- Fixed-size binaries (IDs, hashes)
- Numeric types (integers, floats)
- Dictionary-encoded strings (bounded cardinality)
- Temporal types
- Maps/arrays of primitive types

No `large_string` columns exist in these schemas. QED.

### Algorithmic Primitives as Arrow Transforms

**Submodular Selection**:
```python
def submodular_selection(
    context_items: Table,           # ContextItems batch
    embeddings: Table,              # Joinable via chunk_id  
    budget_tokens: int
) -> Table:
    # Compute gains via Arrow compute on numeric columns
    gains = pc.divide(
        pc.multiply(context_items['marginal_gain'], SELECTION_WEIGHTS),
        context_items['cost_tokens']
    )
    
    # Greedy selection via Arrow array operations  
    selection_vector = knapsack_selection(
        gains, 
        context_items['cost_tokens'], 
        budget_tokens
    )
    
    # Zero-copy update: new batch with updated selection flags
    return update_selection_flags(context_items, selection_vector)
```

**Lyapunov Drift-Plus-Penalty**:
```python
def lyapunov_scheduling(
    queue_state: Table,             # QueueState snapshot  
    tasks: Table                    # Queued tasks
) -> Table:
    # Compute queue drift using Arrow compute
    drift = pc.multiply(
        queue_state['backlog_tokens'],
        queue_state['service_rate_tokens_per_s']
    )
    
    # Priority-weighted task selection
    priority_weights = pc.multiply(
        tasks['expected_cost_tokens'], 
        queue_state['priority_weight']  # Broadcast join
    )
    
    # Output scheduling decisions as new Tasks rows
    return select_tasks_for_execution(tasks, priority_weights)
```

### Evolutionary Stability

The model supports schema evolution through:

1. **Additive-Only Modifications**: New columns never break existing readers
2. **Semantic Versioning**: `schema_version` enables phased migration  
3. **Dictionary Extensibility**: New enumerations expand without structural changes
4. **Type Stability**: Core spine types remain fixed across versions

This unified ontology demonstrates that a single Arrow-native data model can satisfy competing demands of expressive power, structural integrity, privacy enforcement, and algorithmic efficiency for autonomous agent systems.
