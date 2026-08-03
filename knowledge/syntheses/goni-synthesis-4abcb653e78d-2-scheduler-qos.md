---
id: GONI-SYNTHESIS-4ABCB653E78D
title: 2) Scheduler / QoS
type: synthesis
status: draft
implementation_state: specified_only
proposition: time-to-preempt (interactive arrives -> background yields) worst-case non-preemptible region (quantum) cancel-to-quiescent (no more tokens, tools stopped, resources released) cancel-to-safe-state (rollback/compensation if supported) TTFT inflation factor = TTFT(mixed) / TTFT(interactive-only) queue wait time and background WIP
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 2) Scheduler / QoS
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 2) Scheduler / QoS

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2) Scheduler / QoS

- time-to-preempt (interactive arrives -> background yields)
- worst-case non-preemptible region (quantum)
- cancel-to-quiescent (no more tokens, tools stopped, resources released)
- cancel-to-safe-state (rollback/compensation if supported)
- TTFT inflation factor = TTFT(mixed) / TTFT(interactive-only)
- queue wait time and background WIP
- drop/defer rate for background tasks under load
