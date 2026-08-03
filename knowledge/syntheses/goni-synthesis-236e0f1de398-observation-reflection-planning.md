---
id: GONI-SYNTHESIS-236E0F1DE398
title: Observation → Reflection → Planning
type: synthesis
status: draft
implementation_state: specified_only
proposition: Generative Agents (Park et al., 2023) demonstrates that long-horizon coherence requires an explicit Observation–Reflection–Planning loop.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Observation → Reflection → Planning
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Observation → Reflection → Planning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Observation → Reflection → Planning

Generative Agents (Park et al., 2023) demonstrates that long-horizon coherence requires an explicit Observation–Reflection–Planning loop. Goni encodes this as a Control/Memory-plane obligation: raw events land in the Arrow spine (Observation), periodic jobs distill reflections/long-term facts (Reflection), and planners use both current state and reflections to schedule actions and suggestions (Planning).
