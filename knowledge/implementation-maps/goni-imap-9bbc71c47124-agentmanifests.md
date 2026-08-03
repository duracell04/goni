---
id: GONI-IMAP-9BBC71C47124
title: AgentManifests
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Schema version: **MANIFEST-02** (supersedes MANIFEST-01).'
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
  heading: AgentManifests
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# AgentManifests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### AgentManifests
- Schema version: **MANIFEST-02** (supersedes MANIFEST-01).
- PK: `manifest_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `version: utf8`, `manifest_hash: fixed_size_binary[32]`, `manifest_uri: utf8`, `triggers: map<utf8, utf8>`, `capabilities: map<utf8, utf8>`, `budgets: map<utf8, utf8>`, `ui_surfaces: list<utf8>?`, `identity_requirements: list<utf8>?`, `remote_access: bool?`, `tools: list<utf8>`, `policy_hash: fixed_size_binary[32]`, `state_snapshot_id: fixed_size_binary[16]`, `provenance: map<utf8, utf8>`
- Notes: Semantics live in `blueprint/30-specs/agent-manifest.md`. Optional fields default to empty lists and `false` when null.
