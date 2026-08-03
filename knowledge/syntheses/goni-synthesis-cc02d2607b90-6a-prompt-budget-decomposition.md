---
id: GONI-SYNTHESIS-CC02D2607B90
title: 6a) Prompt-budget decomposition
type: synthesis
status: draft
implementation_state: specified_only
proposition: History tokens carried from prior turns.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: 6a) Prompt-budget decomposition
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# 6a) Prompt-budget decomposition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6a) Prompt-budget decomposition
- History tokens carried from prior turns.
- Context tokens selected by retrieval/context assembly.
- Tool schema tokens added by tool declarations or function signatures.
- Output tokens produced for the visible answer.
- Branch count in the assembled prompt/plan.
- Variant count requested by the user or control plane.
- Reasoning-token usage only when the active runtime/provider exposes it
  directly; hidden internal tokens must not be inferred or fabricated.
