---
id: GONI-SPEC-2BC118CF532B
title: 4. Degradation modes
type: specification
status: draft
implementation_state: specified_only
proposition: 'The kernel exposes explicit modes: Eco Normal Boost Thermal throttle Offline-safe Mode changes adjust budgets, wake rates, compaction thresholds, model tier, context length, verifier budget, draft length, parallel-agent count, and background refresh rate.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 4. Degradation modes
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 4. Degradation modes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Degradation modes

The kernel exposes explicit modes:

- Eco
- Normal
- Boost
- Thermal throttle
- Offline-safe

Mode changes adjust budgets, wake rates, compaction thresholds, model tier,
context length, verifier budget, draft length, parallel-agent count, and
background refresh rate.

When a runtime supports speculative or draft-model inference, the scheduler may
budget the maximum draft prefix, verifier tokens, and confidence cutoff exposed
to the router/control plane. The runtime executes the chosen bundle and reports
capabilities; it does not decide whether to accept, verify, escalate, or send
work to a council.
