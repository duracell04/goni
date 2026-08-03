---
id: GONI-IMAP-B0D4F7D309DD
title: LatentSummaries
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: summary_id = row_id Fields: snapshot_id: fixed_size_binary[16], summary_kind: dict<uint8, utf8>, summary_vector: fixed_size_list<float32>[1536], summary_dim: uint16, summary_hash: fixed_size_binary[32], timestamp: timestamp(ms), agent_id: fixed_size_binary[16], policy_hash: fixed_size_binary[32], state_snapshot_id: fixed_size_binary[16], provenance: map<utf8, utf8>'
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
  heading: LatentSummaries
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# LatentSummaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### LatentSummaries
- PK: `summary_id = row_id`
- Fields: `snapshot_id: fixed_size_binary[16]`, `summary_kind: dict<uint8, utf8>`, `summary_vector: fixed_size_list<float32>[1536]`, `summary_dim: uint16`, `summary_hash: fixed_size_binary[32]`, `timestamp: timestamp(ms)`, `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: `summary_hash` points to a derived artifact stored elsewhere.
