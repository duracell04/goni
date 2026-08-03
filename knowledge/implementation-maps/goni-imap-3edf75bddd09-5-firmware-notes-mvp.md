---
id: GONI-IMAP-3EDF75BDDD09
title: 5. Firmware notes (MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Debounce the button in firmware (and optionally in host).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: 5. Firmware notes (MVP)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 5. Firmware notes (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Firmware notes (MVP)

- Debounce the button in firmware (and optionally in host).
- Define a “safe default state” if the host is down:
  - e.g. dim “idle” after boot, blink on fault.
- Provide a simple bootloader/update path (UF2 for RP2040 is convenient).

---
