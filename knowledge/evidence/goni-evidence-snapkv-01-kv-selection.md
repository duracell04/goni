---
id: GONI-EVIDENCE-SNAPKV-01
title: SnapKV provides an additional model-internal KV selection strategy
type: evidence
status: draft
implementation_state: not_applicable
proposition: "SnapKV reports that prompt-derived attention patterns can be used to select a reduced set of KV positions for later generation, providing an additional L3 KV-residency baseline alongside H2O and paging-based methods."
domains:
- research
- models
- performance
aliases: []
relations:
- type: supports
  target: CONTEXT-ALLOC-01
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-LI2024-SNAPKV
artifacts: []
uncertainty: "SnapKV results depend on model, hardware, context distribution, and task. Its reported accuracy and efficiency should not be generalized to all multi-instruction or agent workloads without reproduction."
legacy: []
---

# SnapKV extends the KV-selection evidence base

SnapKV selects clustered important KV positions per attention head using
attention behavior observed near the end of the prompt.

For Goni this provides another L3 model-internal residency candidate to compare
with recency, paging, H2O-style heavy-hitter retention, and architecture-specific
representation compression.

The important architectural point is that KV selection happens after the prompt
has already entered the model. It therefore cannot substitute for source-level
evidence selection or prompt-level semantic compression.

As with all lossy cognitive-state optimization, KV selection remains outside
the canonical authority path.
