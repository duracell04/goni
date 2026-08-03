---
id: GONI-IMAP-4E9F8B63C8F3
title: 4. Validation checklist
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Measurements we must run on the reference build: Idle power (OS booted, models loaded, no generation) Interactive inference power (typical chat) Sustained load power (long generation / indexing) Continuous cognition power (encoder loop at target cadence) Decoder wake behavior (time-to-first-action and power spike) Sensor ingest overhead (screen capture, mic, camera if enabled)'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/power-and-psu.md
  heading: 4. Validation checklist
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4. Validation checklist

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Validation checklist

Measurements we must run on the reference build:

- Idle power (OS booted, models loaded, no generation)
- Interactive inference power (typical chat)
- Sustained load power (long generation / indexing)
- Continuous cognition power (encoder loop at target cadence)
- Decoder wake behavior (time-to-first-action and power spike)
- Sensor ingest overhead (screen capture, mic, camera if enabled)
- Peak transient behaviour (spike capture if possible)
- PSU thermals and fan behaviour under each regime
- Wall-to-DC efficiency at typical loads (20–60%)

---
