---
id: GONI-IMAP-C3D5D57C4D1A
title: 2. Enforcement
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Schema DSL rejects LargeUtf8 for planes 𝒦 and ℰ at compile time.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/40-privacy-and-text-confinement.md
  heading: 2. Enforcement
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. Enforcement

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Enforcement
- Schema DSL rejects `LargeUtf8` for planes 𝒦 and ℰ at compile time.
- Schema validator scans Arrow `Schema` definitions and fails CI if new `LargeUtf8` columns are added outside 𝒜/𝒳 allowances.
- Manual review: any addition to `51-schemas-mvp.md` must justify text placement and update this theorem if necessary.
