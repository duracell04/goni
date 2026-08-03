---
id: GONI-IMAP-F6D37F7BF27B
title: Requests
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: request_id = row_id Fields: session_id?: fixed_size_binary[16], prompt_hash: fixed_size_binary[32], prompt_tokens_est: uint32, budget_tokens: uint32, task_class: dict<uint8, utf8> Notes: No raw text; hashes only.'
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
  heading: Requests
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Requests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Requests
- PK: `request_id = row_id`
- Fields: `session_id?: fixed_size_binary[16]`, `prompt_hash: fixed_size_binary[32]`, `prompt_tokens_est: uint32`, `budget_tokens: uint32`, `task_class: dict<uint8, utf8>`
- Notes: No raw text; hashes only.
