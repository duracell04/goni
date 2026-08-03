---
id: BOUND-01
title: BOUND-01 - Vision, Memory, and Actuation Boundaries
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: BOUND-01 Status: Specified only / roadmap Goni treats observation, context extraction, memory, and actuation as separate governed capabilities.'
domains:
- memory
- specs
aliases:
- VISION-MEMORY-ACTUATION-BOUNDARIES
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: BOUND-01 - Vision, Memory, and Actuation Boundaries
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# BOUND-01 - Vision, Memory, and Actuation Boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# BOUND-01 - Vision, Memory, and Actuation Boundaries
DOC-ID: BOUND-01
Status: Specified only / roadmap

Goni treats observation, context extraction, memory, and actuation as separate
governed capabilities. Desktop and browser agents often collapse these powers
into one session grant. Goni does not.

The Desktop Agent Firewall is the kernel-mediated boundary that prevents a
visible screen fact from becoming extracted context, durable memory, model
input, synthetic input, external egress, or a side effect without explicit
policy authority.
