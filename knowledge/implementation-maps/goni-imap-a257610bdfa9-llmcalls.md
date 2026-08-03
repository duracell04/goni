---
id: GONI-IMAP-A257610BDFA9
title: LlmCalls
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: call_id = row_id Fields: request_id: fixed_size_binary[16], model_id: dict<uint8, utf8>, prompt_tokens: uint32, completion_tokens: uint32, total_tokens: uint32, latency_ms: uint32, cache_hit: bool Notes: Exact billing; may be linked to spans.'
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
  heading: LlmCalls
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# LlmCalls

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### LlmCalls
- PK: `call_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `model_id: dict<uint8, utf8>`, `prompt_tokens: uint32`, `completion_tokens: uint32`, `total_tokens: uint32`, `latency_ms: uint32`, `cache_hit: bool`
- Notes: Exact billing; may be linked to spans.
