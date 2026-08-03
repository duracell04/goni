---
id: GONI-IMAP-FC6DC389525D
title: Metrics
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: (name, ts, labels) or metric_id = row_id depending on storage Fields: name: dict<uint8, utf8>, value_f64?: float64, value_i64?: int64, labels: map<utf8, utf8> Notes: Prometheus export compatibility; avoid unbounded label cardinality.'
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
  heading: Metrics
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Metrics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Metrics
- PK: `(name, ts, labels)` or `metric_id = row_id` depending on storage
- Fields: `name: dict<uint8, utf8>`, `value_f64?: float64`, `value_i64?: int64`, `labels: map<utf8, utf8>`
- Notes: Prometheus export compatibility; avoid unbounded label cardinality.
