---
id: GONI-IMAP-0FE1A904ED17
title: 1.4 Implementation mapping
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Crate goni-arrow implements \(\mathcal{A}_{rr}^{\text{affine}}\).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 1.4 Implementation mapping
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 1.4 Implementation mapping

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.4 Implementation mapping

- Crate `goni-arrow` implements \(\mathcal{A}_{rr}^{\text{affine}}\).  
- Crate `goni-store` provides persistent functors:
  $$
  \mathrm{Persist} : \mathcal{A} \to \mathcal{A}
  $$
  that map in-memory batches to on-disk segments (Parquet/Lance) and back.  
- Crate `goni-index` provides indexed projections:
  $$
  P : \mathcal{A} \to \mathcal{A}, \quad \text{e.g. } (S \mapsto S') \text{ where } S' \text{ only keeps chunk id + embedding}.
  $$

---
