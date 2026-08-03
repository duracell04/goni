---
id: GONI-IMAP-A2D4D7CE4460
title: 1. Responsibilities (MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Read **power button** input (debounced).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: 1. Responsibilities (MVP)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Responsibilities (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Responsibilities (MVP)

- Read **power button** input (debounced).
- Control a **status bar** (single-colour segments; no RGB effects).
- Expose a simple interface to the host OS (USB recommended):
  - report button events,
  - accept “set status” commands.

Non-goals (MVP):
- fancy animations,
- audio,
- complex sensors.

---
