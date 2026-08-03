---
id: GONI-SYNTHESIS-DC47941EF016
title: Derived user-experience metrics
type: synthesis
status: draft
implementation_state: specified_only
proposition: These are the metrics that define perceived value.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: Derived user-experience metrics
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Derived user-experience metrics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Derived user-experience metrics
These are the metrics that define perceived value.

| Metric | Definition |
| --- | --- |
| Time to first actionable (TTFA) | Time to first actionable output, such as an action card, a concrete next step, or a "I can do X/Y/Z" option. |
| Token gap p95/p99 | Maximum inter-token pause, measured during `decode_stream`. |
| Sustained token rate | Tokens per second while streaming, excluding long tool waits. |
| Tail latency | Response time p95/p99, not just averages. |
