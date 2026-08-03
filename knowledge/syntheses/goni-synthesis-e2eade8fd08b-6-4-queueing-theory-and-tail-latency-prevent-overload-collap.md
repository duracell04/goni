---
id: GONI-SYNTHESIS-E2EADE8FD08B
title: 6.4 Queueing theory and tail latency (prevent overload collapse)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Little''s Law: L = lambda W.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.4 Queueing theory and tail latency (prevent overload collapse)
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.4 Queueing theory and tail latency (prevent overload collapse)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.4 Queueing theory and tail latency (prevent overload collapse)

Little's Law: L = lambda W. If arrival rate grows without bounding work-in-
system, waiting time increases and interactive responsiveness suffers. [R6]

Tail latency dominates perceived responsiveness; systems must isolate
interactive QoS and use percentile SLOs (p95/p99) for TTFT and cancellation. [R7]

Goni mapping (normative):
- Background work must be admission-controlled and preemptible.
- Interactive QoS must be protected under contention.
