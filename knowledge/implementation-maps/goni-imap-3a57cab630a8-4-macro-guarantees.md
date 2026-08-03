---
id: GONI-IMAP-3A57CAB630A8
title: 4. Macro Guarantees
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'SMA: Only tables declared in this block are canonical; missing entries fail build.'
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
  heading: 4. Macro Guarantees
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# 4. Macro Guarantees

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Macro Guarantees
- SMA: Only tables declared in this block are canonical; missing entries fail build.
- TXT: If `plane == Control || plane == Execution` and any field is `LargeUtf8`, compilation fails.
- ZCO: For each table, a `*Batch` type wraps `Arc<RecordBatch>`; public APIs must traffic in these types or opaque IDs.
- Plane enforcement: `plane` tag is fixed per table; mismatches are rejected.
