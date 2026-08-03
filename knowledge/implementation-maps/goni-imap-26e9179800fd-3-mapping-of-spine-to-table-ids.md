---
id: GONI-IMAP-26E9179800FD
title: 3. Mapping of Spine to Table IDs
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '| Table | Domain PK field | Equals row_id?'
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
  heading: 3. Mapping of Spine to Table IDs
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Mapping of Spine to Table IDs

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
