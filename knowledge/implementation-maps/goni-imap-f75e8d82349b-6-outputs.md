---
id: GONI-IMAP-F75E8D82349B
title: 6. Outputs
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Generated Arrow Schema objects (for IPC/Parquet writers).
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/53-schema-dsl-and-macros.md
  heading: 6. Outputs
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# 6. Outputs

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Outputs
- Generated Arrow `Schema` objects (for IPC/Parquet writers).
- Generated Rust newtypes for IDs (e.g., `DocId`, `TaskId`).
- Generated typed batches (e.g., `RequestsBatch`) with field accessors.
- Optional: schema registry JSON for non-Rust consumers.
