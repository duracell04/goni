---
id: GONI-IMAP-D8C62A5CAD22
title: 3. CI Gates
type: implementation-map
status: draft
implementation_state: specified_only
proposition: cargo deny/cargo clippy --deny warnings on goni-schema.
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
  heading: 3. CI Gates
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 3. CI Gates

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. CI Gates
- `cargo deny`/`cargo clippy --deny warnings` on `goni-schema`.
- Schema registry diff (JSON export) must be approved for any PR touching `50-data` or `goni-schema`.
- Docs/tests linkage: a change to `51-schemas-mvp.md` must accompany an update to `53-schema-dsl-and-macros.md`.
