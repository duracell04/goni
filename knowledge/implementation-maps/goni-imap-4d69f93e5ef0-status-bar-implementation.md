---
id: GONI-IMAP-4D69F93E5EF0
title: Status bar implementation
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Two safe options: **Discrete LED segments** (recommended) single-colour LEDs behind a light pipe, driven via PWM channels (or an LED driver IC), looks “calm” and avoids RGB expectations.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: Status bar implementation
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Status bar implementation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Status bar implementation

Two safe options:

1) **Discrete LED segments** (recommended)
- single-colour LEDs behind a light pipe,
- driven via PWM channels (or an LED driver IC),
- looks “calm” and avoids RGB expectations.

2) Addressable LEDs (only if used as single-colour)
- possible, but introduces RGB optics and supply chain variance.

---
