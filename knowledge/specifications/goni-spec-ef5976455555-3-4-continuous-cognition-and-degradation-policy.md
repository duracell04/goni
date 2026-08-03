---
id: GONI-SPEC-EF5976455555
title: 3.4 Continuous Cognition and Degradation Policy
type: specification
status: draft
implementation_state: specified_only
proposition: The system must maintain a lightweight heartbeat loop with configurable cadence (for example 1-10 Hz).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.4 Continuous Cognition and Degradation Policy
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.4 Continuous Cognition and Degradation Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Continuous Cognition and Degradation Policy

- The system must maintain a lightweight heartbeat loop with configurable cadence (for example 1-10 Hz).
- The decoder wake path must use hysteresis and a wake budget (max wakes per minute, minimum cooldown).
- Encoder graphs should be pre-warmed and shape-bounded; the steady-state loop must not trigger compilation.
- Observation sources must support gating; prefer event-driven hooks over constant polling.
- Persistent writes must be governed by a write budget controller (rate limits, significance thresholds, deferred compaction).
- State persistence should follow an LSM-style pattern (in-memory buffers, sequential flushes,
  compaction only under favorable conditions like plugged-in + cool) to reduce write amplification.
- Degradation modes must be explicit and configurable: Eco, Normal, Boost, Thermal throttle, Offline-safe.
