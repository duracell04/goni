---
id: ADAPTATION-LEVELS-01
title: Separate adaptation targets and govern them by reversibility
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should treat memory updates, retrieval changes, workflow or policy changes, adapters, and model-weight changes as distinct adaptation targets with progressively stronger evidence, review, replay, provenance, and rollback requirements.
domains:
- system
- learning
- governance
aliases:
- governed self-evolution
relations:
- type: refines
  target: SYS-03
sources:
- SRC-GAO2026-SELF-EVOLVING-AGENTS
- SRC-CHEN2026-CONTINUAL-EXPERIENCE
artifacts: []
uncertainty: Evidence requirements must be calibrated by consequence and reversibility; the cited continual-internalization failure does not generalize to every learning algorithm.
legacy: []
---

# Separate adaptation targets and govern them by reversibility

Goni already separates fast, medium, and slow forms of learning. This principle makes the research rationale explicit.

## Adaptation targets

1. **Inference-time context** — temporary, cheapest to change, expires with the run.
2. **Persistent memory and retrieval state** — durable but externally inspectable and reversible.
3. **Workflow, prompt, router, validator, or policy artifacts** — system behavior changes that require versioning and replay.
4. **Adapters or scoped learned modules** — parametric changes with narrower declared seams.
5. **Core model weights** — highest-cost, least transparent persistent change.

The system should increase governance as reversibility falls and blast radius rises.

## Promotion discipline

Persistent learning candidates should carry:

- source experience and provenance,
- the proposed generalized change,
- independent evaluation or replay evidence,
- regression and safety checks,
- declared affected scopes,
- rollback or version fallback,
- approval appropriate to consequence.

Experience is evidence for a candidate improvement, not proof that the system should internalize it.

The continual-experience literature provides a concrete falsification warning: repeated internalization can compound errors and reduce capability. Goni therefore defaults to external, inspectable adaptation before weight changes.
