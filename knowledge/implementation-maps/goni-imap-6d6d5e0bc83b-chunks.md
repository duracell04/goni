---
id: GONI-IMAP-6D6D5E0BC83B
title: Chunks
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: chunk_id = row_id Fields: doc_id: fixed_size_binary[16], ordinal: uint32, text: large_utf8, token_count: uint32, section_path: list<utf8> Notes: **Only** raw text column #1 (with Prompts.text in ??).'
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
  heading: Chunks
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Chunks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Chunks
- PK: `chunk_id = row_id`
- Fields: `doc_id: fixed_size_binary[16]`, `ordinal: uint32`, `text: large_utf8`, `token_count: uint32`, `section_path: list<utf8>`
- Notes: **Only** raw text column #1 (with Prompts.text in ??).
