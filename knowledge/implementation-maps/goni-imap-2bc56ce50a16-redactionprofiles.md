---
id: GONI-IMAP-2BC56CE50A16
title: RedactionProfiles
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: redaction_profile_id = row_id Fields: name: utf8, mode: dict<uint8, utf8>, ruleset_hash: fixed_size_binary[32], created_at: timestamp(ms)'
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
  heading: RedactionProfiles
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# RedactionProfiles

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### RedactionProfiles
- PK: `redaction_profile_id = row_id`
- Fields: `name: utf8`, `mode: dict<uint8, utf8>`, `ruleset_hash: fixed_size_binary[32]`, `created_at: timestamp(ms)`
