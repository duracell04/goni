---
id: GONI-SPEC-637B09F45A99
title: 1. Purpose
type: specification
status: draft
implementation_state: specified_only
proposition: The Latent State Store (LSS) is the kernel primitive that maintains "what is going on" without requiring continuous LLM decoding.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 1. Purpose
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose

The Latent State Store (LSS) is the kernel primitive that maintains "what is
going on" without requiring continuous LLM decoding. It is the canonical source
for:

- `S_core`: small dense working state (hot, always resident).
- `Delta` stream: append-only updates for reconstruction.
- `F_sparse`: keyed facts/flags (typed, symbolic).

All state changes are policy-checked and auditable.
