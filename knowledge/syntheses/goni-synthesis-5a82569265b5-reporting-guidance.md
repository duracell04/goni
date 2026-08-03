---
id: GONI-SYNTHESIS-5A82569265B5
title: Reporting guidance
type: synthesis
status: draft
implementation_state: specified_only
proposition: Always report p50/p95/p99 for TTFT and response time.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: Reporting guidance
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Reporting guidance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Reporting guidance
- Always report p50/p95/p99 for TTFT and response time.
- Separate compute time from waiting time (queue, tools, retrieval).
- Distinguish "model tokens" from "heartbeat chunks."
- When reporting prompt budgets, separate history/context/tool-schema/output
  token classes whenever observable.
- Hidden reasoning-token accounting must not be inferred unless the runtime or
  provider exposes it directly.
