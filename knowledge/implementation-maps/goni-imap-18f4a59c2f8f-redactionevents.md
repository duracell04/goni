---
id: GONI-IMAP-18F4A59C2F8F
title: RedactionEvents
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: redaction_event_id = row_id Fields: request_id: fixed_size_binary[16], redaction_profile_id: fixed_size_binary[16], timestamp: timestamp(ms), before_hash: fixed_size_binary[32], after_hash: fixed_size_binary[32], redaction_summary: map<utf8, utf8> Notes: redaction_summary must not contain raw prompt text.'
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
  heading: RedactionEvents
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# RedactionEvents

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### RedactionEvents
- PK: `redaction_event_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `redaction_profile_id: fixed_size_binary[16]`, `timestamp: timestamp(ms)`,
  `before_hash: fixed_size_binary[32]`, `after_hash: fixed_size_binary[32]`, `redaction_summary: map<utf8, utf8>`
- Notes: `redaction_summary` must not contain raw prompt text.
