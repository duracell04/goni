---
id: GONI-IMAP-E87BEE0DFFDA
title: ContextItems
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: context_item_id = row_id Fields: context_id: fixed_size_binary[16], chunk_id: fixed_size_binary[16], cost_tokens: uint32, selected: bool, rank: uint16?, marginal_gain: float32?'
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
  heading: ContextItems
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# ContextItems

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### ContextItems
- PK: `context_item_id = row_id`
- Fields: `context_id: fixed_size_binary[16]`, `chunk_id: fixed_size_binary[16]`, `cost_tokens: uint32`, `selected: bool`, `rank: uint16?`, `marginal_gain: float32?`
- Notes: Submodular selection outputs `selected`/`rank`; joins to ?? via `chunk_id`.
