---
id: GONI-IMAP-1F29CA2824F2
title: 5. MVP vs future VecDB
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**MVP** Single-node index with modest document set.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 5. MVP vs future VecDB
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 5. MVP vs future VecDB

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. MVP vs future VecDB

**MVP**

* Single-node index with modest document set.
* One backend (e.g. DuckDB+Lance, Qdrant, etc.) is sufficient.

**Future**

* Sharded / partitioned indices across nodes in the mesh.
* Hybrid lexical + vector retrieval.
* Rich filtering (by source, time, tags) at VecDB level.
