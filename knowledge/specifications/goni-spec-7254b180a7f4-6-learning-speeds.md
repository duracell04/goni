---
id: GONI-SPEC-7254B180A7F4
title: 6. Learning speeds
type: specification
status: draft
implementation_state: specified_only
proposition: 'Fast learning happens in the harness and memory layer: Slow learning MAY create preference datasets for adapter, LoRA, or DPO-style batch updates only after enough evidence, evaluation, and promotion review.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 6. Learning speeds
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 6. Learning speeds

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Learning speeds

Fast learning happens in the harness and memory layer:

```text
correction delta
-> memory rule
-> retrieval / prompt / harness policy update
-> regression test
-> receipt
```

Slow learning MAY create preference datasets for adapter, LoRA, or DPO-style
batch updates only after enough evidence, evaluation, and promotion review.

Core model or constitutional defaults are very slow-moving and require explicit
approval. CDC MUST NOT imply online base-model weight updates in production.
