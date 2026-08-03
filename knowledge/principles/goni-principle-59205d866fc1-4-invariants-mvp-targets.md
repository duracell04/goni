---
id: GONI-PRINCIPLE-59205D866FC1
title: 4. Invariants & MVP targets
type: principle
status: draft
implementation_state: specified_only
proposition: '**Arrow Spine invariant** search returns an Arrow RecordBatch wired into ??; no JSON/serde in the hot path.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/vecdb.md
  heading: 4. Invariants & MVP targets
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 4. Invariants & MVP targets

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Invariants & MVP targets

* **Arrow Spine invariant**
  search returns an Arrow RecordBatch wired into ??; no JSON/serde in the hot path.

* **Quality target (recall)**
  For small synthetic benchmarks, ANN recall@K vs brute-force = 0.9.

* **Latency target**
  For typical K (e.g. 64–128) and dataset sizes expected for a single user, p99 search latency « LLM latency (target < 50 ms).

* **Freshness**
  New chunks should be searchable “soon enough” (MVP: after a bounded delay, or on explicit 
ebuild).

---
