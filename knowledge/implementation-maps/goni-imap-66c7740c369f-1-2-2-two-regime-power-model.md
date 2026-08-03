---
id: GONI-IMAP-66C7740C369F
title: 1.2.2 Two-regime power model
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Goni operates in two regimes: Continuous cognition (always-on encoders + predictor).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.2.2 Two-regime power model
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.2.2 Two-regime power model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2.2 Two-regime power model

Goni operates in two regimes:

- Continuous cognition (always-on encoders + predictor).
- Reasoning bursts (solver/ITCR interrupts).

The product requirement is to minimize solver duty cycle while preserving burst
responsiveness without destabilizing thermals. Hardware MUST support stable
operation under both regimes, with telemetry to distinguish them.
