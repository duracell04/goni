---
id: GONI-IMAP-1BD31DE763C5
title: 1. Text Confinement Theorem
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The only columns in the system with LargeUtf8 and average length > 512 bytes are: Chunks.text Prompts.text All other columns are bounded ≤ 256 bytes or numeric/dictionary encoded.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/40-privacy-and-text-confinement.md
  heading: 1. Text Confinement Theorem
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Text Confinement Theorem

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Text Confinement Theorem
The only columns in the system with `LargeUtf8` and average length > 512 bytes are:
- `Chunks.text`
- `Prompts.text`

All other columns are bounded ≤ 256 bytes or numeric/dictionary encoded.

**Corollary:** Any export of 𝒦 ∪ ℰ is safe for analytics/off-device use; no PII-bearing text exists in those planes.
