---
id: EVID-GIM2024-PROMPT-CACHE
title: 'Source claim: Prompt Cache'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Prompt Cache reuses precomputed attention states for explicitly defined recurring prompt modules and reports substantial time-to-first-token reductions without changing model parameters.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-GIM2024-PROMPT-CACHE
artifacts: []
uncertainty: Reported latency gains depend on model, hardware, prompt structure, module reuse, and runtime implementation; they do not establish one universal cache policy for Goni.
legacy: []
---

# Source claim: Prompt Cache

Prompt Cache defines reusable prompt modules and precomputes their attention
states so recurring text segments can reuse inference work while preserving
positional correctness.

## Goni relevance

The result supports treating stable instruction or context modules as
cache-addressable runtime objects when their exact compatibility conditions are
satisfied.

## Boundary

Semantic similarity alone is insufficient evidence of cache compatibility.
Goni must bind reuse to exact model, tokenizer, template, position, adapter, and
runtime assumptions.
