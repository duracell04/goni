---
id: GONI-IMAP-034B288EB63B
title: 1. Submodular Selection (Context Window)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Inputs: 𝒜.Embeddings.vector, 𝒜.Chunks.text (for final take), 𝒳.ContextItems (cost, marginal_gain).'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/52-zero-copy-mechanics.md
  heading: 1. Submodular Selection (Context Window)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Submodular Selection (Context Window)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Submodular Selection (Context Window)
- Inputs: 𝒜.Embeddings.vector, 𝒜.Chunks.text (for final take), 𝒳.ContextItems (cost, marginal_gain).
- Process: compute gains; greedy/lazy-greedy knapsack; update `ContextItems.selected` and `ContextItems.rank`.
- Output: mask drives an Arrow `take` on `Chunks.text`. Copies: 0.
