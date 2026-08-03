---
id: GONI-SPEC-F5F4A27519D7
title: 2. Hysteresis and wake control
type: specification
status: draft
implementation_state: specified_only
proposition: 'To prevent thrash: minimum cooldown between solver wakes, max solver wakes per time window, rising and falling thresholds for surprisal.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 2. Hysteresis and wake control
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 2. Hysteresis and wake control

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Hysteresis and wake control

To prevent thrash:

- minimum cooldown between solver wakes,
- max solver wakes per time window,
- rising and falling thresholds for surprisal.
