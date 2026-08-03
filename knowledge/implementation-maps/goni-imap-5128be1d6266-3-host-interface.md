---
id: GONI-IMAP-5128BE1D6266
title: 3. Host interface
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Preferred: **USB HID** simplest for “button + status” semantics, no drivers required for basic input.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: 3. Host interface
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Host interface

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Host interface

Preferred: **USB HID**
- simplest for “button + status” semantics,
- no drivers required for basic input.

Alternative: USB serial (CDC-ACM)
- easy for debugging, but needs a small daemon/service.

Suggested commands (conceptual):
- SET_STATE: idle / thinking / busy / update / error
- SET_BRIGHTNESS: 0–100
- PULSE: brief attention pulse (optional)

---
