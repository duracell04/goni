---
id: EVID-DENNING1968-WORKING-SET
title: 'Source claim: Denning 1968 working-set model'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Denning's working-set model treats the subset of information currently in use by a running process as the basis for dynamic memory allocation rather than keeping the entire addressable state resident.
domains:
- research
- system
aliases: []
relations:
- type: supports
  target: GONI-PRINCIPLE-9062425CD490
- type: supports
  target: GONI-DECISION-FEB39440C565
sources:
- SRC-DENNING1968-WORKING-SET
artifacts: []
uncertainty: The working-set model is an operating-systems result. Applying it to LLM context management is an architectural analogy that requires Goni-specific measurement rather than a direct empirical result from Denning.
legacy: []
---

# Source claim: Denning 1968 working-set model

Denning develops the working-set model to distinguish information currently in
use by a running process from the larger addressable memory space and to guide
dynamic management of paged memory.

## Goni relevance

This supports treating Goni's durable memory and addressable knowledge as larger
than the active inference working set. It motivates dynamic admission, eviction,
and paging policies while leaving the exact LLM-specific policy to empirical
evaluation.

## Boundary

The paper does not study transformers, retrieval-augmented generation, prompt
compression, or KV caches. Goni's use of the working-set concept is a systems
analogy, not a claim that LLM context obeys the same locality distribution.
