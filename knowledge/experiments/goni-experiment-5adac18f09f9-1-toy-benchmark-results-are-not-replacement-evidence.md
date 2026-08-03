---
id: GONI-EXPERIMENT-5ADAC18F09F9
title: 1. Toy benchmark results are not replacement evidence
type: experiment
status: draft
implementation_state: not_applicable
proposition: Small-model wins on toy datasets are useful for ablation, not sufficient for claims of practical transformer replacement.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: 1. Toy benchmark results are not replacement evidence
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# 1. Toy benchmark results are not replacement evidence

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Toy benchmark results are not replacement evidence

Small-model wins on toy datasets are useful for ablation, not sufficient for
claims of practical transformer replacement.

Evaluation implication:
- treat tiny char-level WikiText-2 results as proof-of-concept only,
- require scale-up evidence on realistic corpora and model sizes before
  architecture-level conclusions.

Related work:
- S4 long-sequence evaluation framing [[gu2021-s4]]
- RWKV scaling evidence to larger model sizes [[peng2023-rwkv]]
- Hyena hierarchy large-scale comparisons [[poli2023-hyena]].
