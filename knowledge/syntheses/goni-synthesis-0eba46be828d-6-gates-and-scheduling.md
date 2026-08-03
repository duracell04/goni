---
id: GONI-SYNTHESIS-0EBA46BE828D
title: 6) Gates and Scheduling
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: Purpose: non-bypassable mediation and predictable QoS.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/10-primitives.md
  heading: 6) Gates and Scheduling
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 6) Gates and Scheduling

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6) Gates and Scheduling
- Purpose: non-bypassable mediation and predictable QoS.
- Contract anchors: `30-specs/tool-capability-api.md`, `30-specs/network-gate-and-anonymity.md`, `30-specs/scheduler-and-interrupts.md`.
- Core invariant: no ambient authority; interactive work preempts background work.
- Metrics: preemption latency, cancel-to-quiescent, blocked egress rate.
