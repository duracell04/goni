---
id: GONI-IMAP-C81B874637DF
title: 4. Evolution Policy
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Additive-only column changes; defaults to null for new fields.
domains:
- data
- software
- validation
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/80-validation-and-ci.md
  heading: 4. Evolution Policy
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 4. Evolution Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Evolution Policy
- Additive-only column changes; defaults to null for new fields.
- `schema_version` bump on semantic changes; old binaries must read new files.
- Dictionary expansions are allowed without structural change.
