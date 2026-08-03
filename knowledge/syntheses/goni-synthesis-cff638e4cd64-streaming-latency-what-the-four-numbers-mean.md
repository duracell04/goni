---
id: GONI-SYNTHESIS-CFF638E4CD64
title: 'Streaming latency: what the four numbers mean'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Think of a request as a timeline: t0 (user sends) -> networking -> queue/scheduler -> prefill -> decode tokens -> optional tools/RAG -> more decode -> stream ends.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 'Streaming latency: what the four numbers mean'
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# Streaming latency: what the four numbers mean

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Streaming latency: what the four numbers mean

Think of a request as a timeline:

`t0 (user sends)` -> networking -> queue/scheduler -> prefill -> decode tokens
-> optional tools/RAG -> more decode -> stream ends.
