---
id: GONI-SYNTHESIS-A043EB820D17
title: 2) Scheduler and isolation
type: synthesis
status: draft
implementation_state: specified_only
proposition: Time-to-preempt (interactive arrival to background yield).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: 2) Scheduler and isolation
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# 2) Scheduler and isolation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2) Scheduler and isolation
- Time-to-preempt (interactive arrival to background yield).
- Cancellation latency (cancel to quiescent, cancel to safe state).
- Mixed workload inflation factor (TTFT mixed / TTFT idle).
