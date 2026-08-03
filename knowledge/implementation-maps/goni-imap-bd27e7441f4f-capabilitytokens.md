---
id: GONI-IMAP-BD27E7441F4F
title: CapabilityTokens
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: capability_token_id = row_id Fields: agent_id: fixed_size_binary[16], policy_hash: fixed_size_binary[32], tools: list<utf8>, fs_read_roots: list<utf8>, fs_write_roots: list<utf8>, net_allowlist: list<utf8>, budgets: map<utf8, utf8>, issued_at: timestamp(ms), expires_at: timestamp(ms), provenance: map<utf8, utf8>'
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
  heading: CapabilityTokens
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# CapabilityTokens

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### CapabilityTokens
- PK: `capability_token_id = row_id`
- Fields: `agent_id: fixed_size_binary[16]`, `policy_hash: fixed_size_binary[32]`, `tools: list<utf8>`, `fs_read_roots: list<utf8>`, `fs_write_roots: list<utf8>`, `net_allowlist: list<utf8>`, `budgets: map<utf8, utf8>`, `issued_at: timestamp(ms)`, `expires_at: timestamp(ms)`, `provenance: map<utf8, utf8>`
