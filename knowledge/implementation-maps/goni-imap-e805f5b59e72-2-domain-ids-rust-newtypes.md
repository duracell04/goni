---
id: GONI-IMAP-E805F5B59E72
title: 2. Domain IDs (Rust newtypes)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Each canonical table exposes a newtype around [u8; 16]/Uuid: DocId, ChunkId, EmbeddingId PromptId, ContextItemId RequestId, TaskId AuditId, CapabilityTokenId, RedactionProfileId, RedactionEventId, AgentManifestId SnapshotId, DeltaId, SummaryId, MemoryId CallId, SignalId, CapabilityId, MetricId Each equals Spine.row_id for its table.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/20-spine-and-ids.md
  heading: 2. Domain IDs (Rust newtypes)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. Domain IDs (Rust newtypes)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Domain IDs (Rust newtypes)

Each canonical table exposes a newtype around `[u8; 16]`/`Uuid`:

- `DocId`, `ChunkId`, `EmbeddingId`
- `PromptId`, `ContextItemId`
- `RequestId`, `TaskId`
- `AuditId`, `CapabilityTokenId`, `RedactionProfileId`, `RedactionEventId`, `AgentManifestId`
- `SnapshotId`, `DeltaId`, `SummaryId`, `MemoryId`
- `CallId`, `SignalId`, `CapabilityId`, `MetricId`

Each equals `Spine.row_id` for its table. No other ID representations are allowed across crate boundaries.
