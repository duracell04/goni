---
id: GONI-SYNTHESIS-18BCBE80AA6F
title: 7) Add a research-first long-context reading harness
type: synthesis
status: draft
implementation_state: specified_only
proposition: Add corpus fixtures and gold answers for long single-doc, multi-doc, span extraction, and needle-in-corpus tasks.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 7) Add a research-first long-context reading harness
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 7) Add a research-first long-context reading harness

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7) Add a research-first long-context reading harness
- Add corpus fixtures and gold answers for long single-doc, multi-doc, span
  extraction, and needle-in-corpus tasks.
- Add a research harness that can compare:
  - full-context baseline,
  - current RAG/context assembly baseline,
  - programmatic corpus-reading baseline,
  - hybrid retrieval + corpus-reading baseline.
- Produce one operator-facing comparison report for quality, cost, latency, and
  failure modes.
- Keep stories INVEST-sized:
  - one fixture family,
  - one strategy comparison,
  - one measurable output per slice.
