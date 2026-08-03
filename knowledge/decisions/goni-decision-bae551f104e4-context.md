---
id: GONI-DECISION-BAE551F104E4
title: Context
type: decision
status: draft
implementation_state: specified_only
proposition: 'The MVP experience target includes: two local models in parallel (8â€“14B quant), interactive context lengths (8â€“16k), RAG indexes and embeddings in memory.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Context
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Context

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Context

The MVP experience target includes:

- two local models in parallel (8â€“14B quant),
- interactive context lengths (8â€“16k),
- RAG indexes and embeddings in memory.

64 GB unified memory can run smaller models, but it is not representative of the â€œreal exocortexâ€ story and tends to collapse under concurrency.
