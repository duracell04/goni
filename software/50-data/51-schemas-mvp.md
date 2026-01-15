# 51 – Schemas (MVP Canonical Tables)
DOC-ID: SCHEMA-MVP-01

Arrow-first, v1.0 schemas for the canonical tables. Each table is `Spine + Payload`; `row_id` == domain PK.

**Executable spec:** These schemas are implemented by `software/kernel/goni-schema` via the `define_tables!` block in `goni-schema/src/lib.rs`. This document and that DSL must stay in sync.

## Plane ?? – Knowledge (immutable)

### Docs
- PK: `doc_id = row_id`
- Fields: `source_uri: large_utf8`, `mime_type: utf8`, `title: utf8`, `tags: list<utf8>`, `metadata: map<utf8, utf8>`
- Notes: No full text beyond metadata; content lives in Chunks.

### Chunks
- PK: `chunk_id = row_id`
- Fields: `doc_id: fixed_size_binary[16]`, `ordinal: uint32`, `text: large_utf8`, `token_count: uint32`, `section_path: list<utf8>`
- Notes: **Only** raw text column #1 (with Prompts.text in ??).

### Embeddings
- PK: `embedding_id = row_id`
- Fields: `chunk_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `vector: fixed_size_list<float32>[1536]`, `dim: uint16`
- Notes: Lance index on `vector`.

## Plane ?? – Context (ephemeral)

### Prompts
- PK: `prompt_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `source_context_id: fixed_size_binary[16]`, `timestamp: timestamp(ms)`,
  `materialization_kind: dict<uint8, utf8>`, `prompt_hash: fixed_size_binary[32]`,
  `token_estimate_in: uint32`, `token_estimate_out: uint32`, `is_redacted: bool`,
  `redaction_profile_id?: fixed_size_binary[16]`, `text?: large_utf8`
- Notes: `text` may be null; if present, it is the only raw prompt text in the system.

### ContextItems
- PK: `context_item_id = row_id`
- Fields: `context_id: fixed_size_binary[16]`, `chunk_id: fixed_size_binary[16]`, `cost_tokens: uint32`, `selected: bool`, `rank: uint16?`, `marginal_gain: float32?`
- Notes: Submodular selection outputs `selected`/`rank`; joins to ?? via `chunk_id`.

## Plane ?? – Control (metadata only)

### Requests
- PK: `request_id = row_id`
- Fields: `session_id?: fixed_size_binary[16]`, `prompt_hash: fixed_size_binary[32]`, `prompt_tokens_est: uint32`, `budget_tokens: uint32`, `task_class: dict<uint8, utf8>`
- Notes: No raw text; hashes only.

### Tasks
- PK: `task_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `task_type: dict<uint8, utf8>`, `state: dict<uint8, utf8>`, `queue_id: dict<uint8, utf8>`, `expected_cost_tokens: uint32`
- Notes: Lyapunov inputs; append-only state transitions.

## Plane E – Execution (telemetry)

### LlmCalls
- PK: `call_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `prompt_tokens: uint32`, `completion_tokens: uint32`, `total_tokens: uint32`, `latency_ms: uint32`, `cache_hit: bool`
- Notes: Exact billing; may be linked to spans.

### PlatformSignals
- PK: `signal_id = row_id`
- Fields: `timestamp: timestamp(ms)`, `device_id: fixed_size_binary[16]`, `session_id?: fixed_size_binary[16]`,
  `thermal_throttled?: bool`, `thermal_domain?: dict<uint8, utf8>`, `dvfs_state?: dict<uint8, utf8>`,
  `free_ram_mb?: uint32`, `swap_in_mb?: uint32`, `major_faults?: uint32`,
  `bytes_written_today?: int64`, `waf_estimate?: float32`, `ssd_health?: float32`,
  `npu_shape_buckets?: list<utf8>`, `supported_quant?: list<utf8>`,
  `gpu_active?: bool`, `gpu_wake_ms_p95?: uint32`,
  `solver_wake_count?: uint32`, `solver_active_ms?: uint32`, `encoder_active_ms?: uint32`
- Notes: Optional fields support partial telemetry. Use dict enums for domain/state tags.

### PlatformCapabilities
- PK: `capability_id = row_id`
- Fields: `timestamp: timestamp(ms)`, `device_id: fixed_size_binary[16]`,
  `npu_shape_buckets?: list<utf8>`, `supported_quant?: list<utf8>`
- Notes: Static or infrequently changing capability snapshot.

### Metrics
- PK: `(name, ts, labels)` or `metric_id = row_id` depending on storage
- Fields: `name: dict<uint8, utf8>`, `value_f64?: float64`, `value_i64?: int64`, `labels: map<utf8, utf8>`
- Notes: Prometheus export compatibility; avoid unbounded label cardinality.

## Plane ?? - Memory (lifecycle-aware)

### MemoryEntries
- PK: `memory_id = row_id`
- Fields: `kind: dict<uint8, utf8> (fact|preference|decision|hypothesis|derived)`, `timestamp: timestamp(ms)`,
  `value: map<utf8, utf8>`, `confidence: float32`, `source_chunk_ids: list<utf8>`,
  `confirmed_by_event_id?: fixed_size_binary[16]`, `review_at?: timestamp(ms)`, `ttl_ms?: uint32`,
  `conflict_state?: dict<uint8, utf8>`, `embedding?: fixed_size_list<float32>[1536]`, `embedding_dim?: uint16`
- Notes: `source_chunk_ids` values are UUIDv7 strings referencing `Chunks.chunk_id`; raw text is not stored here.

## Plane ?? - Knowledge (latent state)

### StateSnapshots
- PK: `snapshot_id = row_id`
- Fields: `state_version: uint32`, `s_core: fixed_size_list<float32>[1536]`, `s_core_dim: uint16`, `f_sparse: map<utf8, utf8>`, `created_at: timestamp(ms)`, `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: `state_snapshot_id` equals `snapshot_id` for snapshots.

### StateDeltas
- PK: `delta_id = row_id`
- Fields: `snapshot_id: fixed_size_binary[16]`, `delta_kind: dict<uint8, utf8>`, `delta_vector: fixed_size_list<float32>[1536]`, `delta_dim: uint16`, `f_sparse_patch: map<utf8, utf8>`, `timestamp: timestamp(ms)`, `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: Deltas are append-only and ordered by timestamp.

#### F_sparse conventions (SS-01)
- f_sparse and f_sparse_patch remain map<utf8, utf8>; storage treats values as opaque.
- Keys use namespaces: policy.*, goal.*, constraint.*, fact.*.
- Values are JSON objects with a required version field v.
- Example (single row):
```json
{
  "policy.no_send_email": "{\"v\":1,\"effect\":\"deny\",\"subject\":{\"tool_id\":\"email.send\"},\"on_fail\":\"block\"}",
  "goal.next_action": "{\"v\":1,\"kind\":\"draft\",\"target\":\"email\"}",
  "constraint.requires_source": "{\"v\":1,\"effect\":\"deny\",\"subject\":{\"tool_id\":\"fs.write\"},\"when\":{\"op\":\"missing\",\"args\":[\"fact.source_ref\"]},\"on_fail\":\"ask\"}",
  "fact.user_tier": "{\"v\":1,\"value\":\"local\"}"
}
```
- Validation semantics live in docs/specs/symbolic-substrate.md; storage does not enforce schemas.

### LatentSummaries
- PK: `summary_id = row_id`
- Fields: `snapshot_id: fixed_size_binary[16]`, `summary_kind: dict<uint8, utf8>`, `summary_vector: fixed_size_list<float32>[1536]`, `summary_dim: uint16`, `summary_hash: fixed_size_binary[32]`, `timestamp: timestamp(ms)`, `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: `summary_hash` points to a derived artifact stored elsewhere.

## Plane ?? - Control (policy and audit)

### AuditRecords
- PK: `audit_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `capability_token_id: fixed_size_binary[16]`, `tool_id: dict<uint8, utf8>`, `args_hash: fixed_size_binary[32]`, `result_hash: fixed_size_binary[32]`, `timestamp: timestamp(ms)`, `provenance: map<utf8, utf8>`

### CapabilityTokens
- PK: `capability_token_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `tools: list<utf8>`, `fs_read_roots: list<utf8>`, `fs_write_roots: list<utf8>`, `net_allowlist: list<utf8>`, `budgets: map<utf8, utf8>`, `issued_at: timestamp(ms)`, `expires_at: timestamp(ms)`, `provenance: map<utf8, utf8>`

### RedactionProfiles
- PK: `redaction_profile_id = row_id`
- Fields: `name: utf8`, `mode: dict<uint8, utf8>`, `ruleset_hash: fixed_size_binary[32]`, `created_at: timestamp(ms)`

### RedactionEvents
- PK: `redaction_event_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `redaction_profile_id: fixed_size_binary[16]`, `timestamp: timestamp(ms)`,
  `before_hash: fixed_size_binary[32]`, `after_hash: fixed_size_binary[32]`, `redaction_summary: map<utf8, utf8>`
- Notes: `redaction_summary` must not contain raw prompt text.

### AgentManifests
- Schema version: **MANIFEST-02** (supersedes MANIFEST-01).
- PK: `manifest_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `version: utf8`, `manifest_hash: fixed_size_binary[32]`, `manifest_uri: utf8`, `triggers: map<utf8, utf8>`, `capabilities: map<utf8, utf8>`, `budgets: map<utf8, utf8>`, `ui_surfaces: list<utf8>?`, `identity_requirements: list<utf8>?`, `remote_access: bool?`, `tools: list<utf8>`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: Semantics live in `docs/specs/agent-manifest.md`. Optional fields default to empty lists and `false` when null.

## Invariants
- All IDs are `fixed_size_binary[16]` (UUIDv7) and equal `Spine.row_id` for their table.
- No `LargeUtf8` outside `Chunks.text` and `Prompts.text` (TXT axiom).
- Dictionaries enumerate finite vocabularies; adding a new label/value requires schema version bump.
- Latent state, audit, and manifest records include `agent_id`, `policy_hash`,
  `state_snapshot_id`, and `provenance` (directly or by reference).
