---
id: GONI-IMAP-5C80000D9992
title: 2. Schema Tests
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Unit test walks all generated Arrow Schema objects, asserting: plane tags match file placement, IDs are FixedSizeBinary(16), and text confinement holds.'
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
  heading: 2. Schema Tests
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 2. Schema Tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Schema Tests
- Unit test walks all generated Arrow `Schema` objects, asserting: plane tags match file placement, IDs are `FixedSizeBinary(16)`, and text confinement holds.
- Round-trip IPC/Parquet tests ensure forward compatibility (additive-only evolution).
