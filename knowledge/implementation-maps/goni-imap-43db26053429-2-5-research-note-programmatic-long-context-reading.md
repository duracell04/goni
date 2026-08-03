---
id: GONI-IMAP-43DB26053429
title: '2.5 Research note: programmatic long-context reading'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: This blueprint may evaluate a separate research lane for **programmatic long-context reading**.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: '2.5 Research note: programmatic long-context reading'
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 2.5 Research note: programmatic long-context reading

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.5 Research note: programmatic long-context reading

This blueprint may evaluate a separate research lane for **programmatic
long-context reading**.

- The default architecture remains submodular context selection over retrieved
  chunks.
- The research lane allows the predictor to inspect external corpora through
  tool-mediated operations such as search, slice, filter, and bounded
  subreads.
- Any such reading strategy remains bounded by scheduler budgets, tool
  mediation, and receipt requirements.
- This is not, in this phase, a claim that programmatic reading replaces the
  Memory Plane, provenance requirements, or the current retrieval baseline.

---
