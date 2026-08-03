---
id: SCHEMA-MVP-01
title: 51 – Schemas (MVP Canonical Tables)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: SCHEMA-MVP-01 Arrow-first, v1.0 schemas for the canonical tables.'
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
  heading: 51 – Schemas (MVP Canonical Tables)
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# 51 – Schemas (MVP Canonical Tables)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# 51 – Schemas (MVP Canonical Tables)
DOC-ID: SCHEMA-MVP-01

Arrow-first, v1.0 schemas for the canonical tables. Each table is `Spine + Payload`; `row_id` == domain PK.

**Executable spec:** These schemas are implemented by `goni-prototype-lab:software/kernel/goni-schema` via the `define_tables!` block in `goni-schema/src/lib.rs`. This document and that DSL must stay in sync.
