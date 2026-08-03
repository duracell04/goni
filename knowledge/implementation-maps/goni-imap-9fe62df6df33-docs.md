---
id: GONI-IMAP-9FE62DF6DF33
title: Docs
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: doc_id = row_id Fields: source_uri: large_utf8, mime_type: utf8, title: utf8, tags: list<utf8>, metadata: map<utf8, utf8> Notes: No full text beyond metadata; content lives in Chunks.'
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
  heading: Docs
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Docs

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Docs
- PK: `doc_id = row_id`
- Fields: `source_uri: large_utf8`, `mime_type: utf8`, `title: utf8`, `tags: list<utf8>`, `metadata: map<utf8, utf8>`
- Notes: No full text beyond metadata; content lives in Chunks.
