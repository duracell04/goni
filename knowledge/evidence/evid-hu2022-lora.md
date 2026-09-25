---
id: EVID-HU2022-LORA
title: 'Source claim: LoRA'
type: evidence
status: draft
implementation_state: not_applicable
proposition: LoRA adapts a frozen pretrained model by learning low-rank parameter updates, reducing the trainable and stored task-specific parameter burden relative to full fine-tuning.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SPEC-9D8A0A7F0F70
sources:
- SRC-HU2022-LORA
artifacts: []
uncertainty: LoRA demonstrates parameter-efficient adaptation, not that a given behavior should be moved from prompts or skills into an adapter; placement remains a Goni governance and evaluation decision.
legacy: []
---

# Source claim: LoRA

LoRA freezes base-model weights and injects trainable low-rank updates for
downstream adaptation.

## Goni relevance

It provides a concrete learned-adaptation layer between prompt-level procedural
instructions and full model-weight replacement.

## Boundary

The source does not establish when Goni should prefer an adapter over explicit
skills, retrieval, deterministic code, or kernel enforcement.
