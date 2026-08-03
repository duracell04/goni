---
id: GONI-EVIDENCE-970E3ACCFAD2
title: 'Source claim: deepseek2026-v4-dspark-hf'
type: evidence
status: draft
implementation_state: not_applicable
proposition: DeepSeek-V4-Pro-DSpark is not a new foundation model; it is the same DeepSeek-V4-Pro checkpoint with an additional speculative decoding module attached.
domains:
- research
aliases: []
relations:
- type: supports
  target: GONI-IMAP-45DA8323C140
sources:
- SRC-DEEPSEEK2026-V4-DSPARK-HF
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[deepseek2026-v4-dspark-hf]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: deepseek2026-v4-dspark-hf

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[deepseek2026-v4-dspark-hf]]
Claim: DeepSeek-V4-Pro-DSpark is not a new foundation model; it is the same
DeepSeek-V4-Pro checkpoint with an additional speculative decoding module
attached.
Relevance:
- Prevents Goni from treating DSpark as a new base-model supplier. It is
  evidence for runtime orchestration around a model, not for replacing Goni's
  model-selection policy.
Used in:
- `blueprint/software/20-architecture.md`
Source:
- https://huggingface.co/deepseek-ai/DeepSeek-V4-Pro-DSpark
