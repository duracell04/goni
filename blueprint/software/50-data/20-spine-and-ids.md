# 20 – Spine & ID Semantics
DOC-ID: SPINE-01

## 1. Universal Spine (Arrow)

```arrow
struct Spine {
  row_id         : fixed_size_binary[16]  // UUIDv7 – globally unique, monotonic
  tenant_id      : fixed_size_binary[16]  // single-node: fixed 000…001
  plane          : uint8                  // 0=𝒜 1=𝒳 2=𝒦 3=ℰ
  kind           : dictionary<uint8, utf8> // table name for debugging
  schema_version : uint16                 // bump on breaking change
  ts_created     : timestamp[ms, UTC]
  ts_valid_from  : timestamp[ms, UTC]     // SCD-2 facts
}
```

Every table is `Spine + Payload`. `row_id` is the canonical primary key; table-specific ID fields are aliases.

## 2. Domain IDs (Rust newtypes)

Each canonical table exposes a newtype around `[u8; 16]`/`Uuid`:

- `DocId`, `ChunkId`, `EmbeddingId`
- `PromptId`, `ContextItemId`
- `RequestId`, `TaskId`
- `AuditId`, `CapabilityTokenId`, `RedactionProfileId`, `RedactionEventId`, `AgentManifestId`
- `SnapshotId`, `DeltaId`, `SummaryId`, `MemoryId`
- `CallId`, `SignalId`, `CapabilityId`, `MetricId`

Each equals `Spine.row_id` for its table. No other ID representations are allowed across crate boundaries.

## 3. Mapping of Spine to Table IDs

| Table        | Domain PK field | Equals `row_id`? |
|--------------|-----------------|------------------|
| Docs         | `doc_id`        | yes |
| Chunks       | `chunk_id`      | yes |
| Embeddings   | `embedding_id`  | yes |
| Prompts      | `prompt_id`     | yes |
| Requests     | `request_id`    | yes |
| Tasks        | `task_id`       | yes |
| ContextItems | `context_item_id` | yes |
| AuditRecords | `audit_id`      | yes |
| CapabilityTokens | `capability_token_id` | yes |
| RedactionProfiles | `redaction_profile_id` | yes |
| RedactionEvents | `redaction_event_id` | yes |
| AgentManifests | `manifest_id` | yes |
| StateSnapshots | `snapshot_id` | yes |
| StateDeltas | `delta_id` | yes |
| LatentSummaries | `summary_id` | yes |
| MemoryEntries | `memory_id` | yes |
| LlmCalls     | `call_id`       | yes |
| PlatformSignals | `signal_id` | yes |
| PlatformCapabilities | `capability_id` | yes |
| Metrics      | `metric_id`     | yes |

## 4. Tenant & Plane Tagging

- `tenant_id` remains fixed for single-node deployments; multi-tenant variants must route every write through this field.
- `plane` is enforced by the schema DSL; mismatched plane/table pairs fail compilation.
