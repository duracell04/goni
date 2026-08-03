---
id: GONI-SYNTHESIS-3C7B7CA4B3D1
title: 6. Product implication for Goni
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni should remain model-backend pluggable.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: 6. Product implication for Goni
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# 6. Product implication for Goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Product implication for Goni

Goni should remain model-backend pluggable. Product trust should not depend on
any single architecture trend.

Practical stance:
- optimize memory architecture (retrieval, compaction, provenance),
- optimize mediation/governance (capabilities, receipts, policy),
- treat sequence-model innovation as a swap-in backend improvement path.

In deployment terms, long-context quality and cost are often constrained by
retrieval quality, KV behavior, and serving systems design as much as by model
family choice [[kwon2023-vllm]] [[wang2024-mmneedle]].
