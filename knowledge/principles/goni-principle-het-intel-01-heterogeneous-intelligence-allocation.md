---
id: GONI-PRINCIPLE-HET-INTEL-01
title: "Heterogeneous Intelligence Allocation"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should allocate operations to the mechanism with comparative advantage, using deterministic computation for exactly specifiable properties, bounded probabilistic models for semantic judgments over defined answer spaces, generative models for open-ended synthesis, and the kernel for authority decisions."
domains:
- models
- system
aliases: []
relations:
- type: supports
  target: GONI-THESIS-7BFB74017D50
- type: refines
  target: MODEL-REG-01
- type: refines
  target: GONI-THESIS-AF4B6A0B4A2D
- type: refines
  target: GONI-IMAP-EB2133E6965D
- type: refines
  target: GONI-DECISION-9AF49466170D
sources:
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "The boundary between probabilistic and deterministic work is task-dependent and can shift as specialized models, conventional algorithms, and hardware improve."
legacy: []
---

# Heterogeneous Intelligence Allocation

Goni should allocate work by comparative advantage rather than treating a
generative language model as the universal computational primitive.

A useful default taxonomy is:

- exact arithmetic, deterministic transformations, validation, and reproducible
  calculations -> deterministic computation;
- structured lookup -> database, index, or retrieval system;
- constraint satisfaction -> solver;
- stable semantic boundaries -> task-specific classifier where justified;
- bounded semantic judgment over a predefined answer space -> probabilistic
  decision model;
- open-ended drafting, synthesis, and explanation -> generative model;
- difficult multi-step deliberation -> reasoning model or governed multi-model
  escalation;
- consequential permission -> kernel policy against canonical authority state;
- unresolved principal-owned value trade-offs -> principal escalation.

These are allocation categories, not permanent model classes. A task may move
between them as evidence, model capability, deterministic tooling, and operating
costs change.

The governing rule is to use the simplest mechanism that preserves the required
quality and assurance properties. Probabilistic models remain valuable where
semantic judgment is genuinely uncertain. Exact state, permissions, schemas,
and directly computable invariants should not depend on stochastic model
behavior merely because a model can express an answer.

This principle strengthens latent-first cognition: language is one projection of
machine state, not the mandatory representation of every intermediate
computation. It also strengthens the authority boundary: model capability can
change how cognition is performed without changing what the system is permitted
to do.
