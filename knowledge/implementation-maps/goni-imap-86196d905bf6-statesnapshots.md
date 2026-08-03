---
id: GONI-IMAP-86196D905BF6
title: StateSnapshots
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: snapshot_id = row_id Fields: state_version: uint32, s_core: fixed_size_list<float32>[1536], s_core_dim: uint16, f_sparse: map<utf8, utf8>, created_at: timestamp(ms), agent_id: fixed_size_binary[16], policy_hash: fixed_size_binary[32], state_snapshot_id: fixed_size_binary[16], provenance: map<utf8, utf8> Notes: state_snapshot_id equals snapshot_id for snapshots.'
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
  heading: StateSnapshots
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# StateSnapshots

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### StateSnapshots
- PK: `snapshot_id = row_id`
- Fields: `state_version: uint32`, `s_core: fixed_size_list<float32>[1536]`, `s_core_dim: uint16`, `f_sparse: map<utf8, utf8>`, `created_at: timestamp(ms)`, `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: `state_snapshot_id` equals `snapshot_id` for snapshots.
