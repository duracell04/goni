---
id: GONI-IMAP-60ACFEB4E8BA
title: StateDeltas
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: delta_id = row_id Fields: snapshot_id: fixed_size_binary[16], delta_kind: dict<uint8, utf8>, delta_vector: fixed_size_list<float32>[1536], delta_dim: uint16, f_sparse_patch: map<utf8, utf8>, timestamp: timestamp(ms), agent_id: fixed_size_binary[16], policy_hash: fixed_size_binary[32], state_snapshot_id: fixed_size_binary[16], provenance: map<utf8, utf8>'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: StateDeltas
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# StateDeltas

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
- Validation semantics live in blueprint/30-specs/symbolic-substrate.md; storage does not enforce schemas.
