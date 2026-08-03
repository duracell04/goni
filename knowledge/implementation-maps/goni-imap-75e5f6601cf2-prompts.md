---
id: GONI-IMAP-75E5F6601CF2
title: Prompts
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: prompt_id = row_id Fields: request_id: fixed_size_binary[16], source_context_id: fixed_size_binary[16], timestamp: timestamp(ms), materialization_kind: dict<uint8, utf8>, prompt_hash: fixed_size_binary[32], token_estimate_in: uint32, token_estimate_out: uint32, is_redacted: bool, redaction_profile_id?: fixed_size_binary[16], text?: large_utf8'
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
  heading: Prompts
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Prompts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Prompts
- PK: `prompt_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `source_context_id: fixed_size_binary[16]`, `timestamp: timestamp(ms)`,
  `materialization_kind: dict<uint8, utf8>`, `prompt_hash: fixed_size_binary[32]`,
  `token_estimate_in: uint32`, `token_estimate_out: uint32`, `is_redacted: bool`,
  `redaction_profile_id?: fixed_size_binary[16]`, `text?: large_utf8`
- Notes: `text` may be null; if present, it is the only raw prompt text in the system.
