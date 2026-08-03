---
id: GONI-IMAP-81AA8DEA415D
title: 3. Power distribution
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Baseline approach: PSU provides standard rails to the APU board (ATX 24-pin or vendor harness).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/power-and-psu.md
  heading: 3. Power distribution
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Power distribution

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Power distribution

Baseline approach:
- PSU provides standard rails to the APU board (ATX 24-pin or vendor harness).
- Use board-native M.2 power for NVMe.
- Front-panel MCU powered from:
  - 5V standby (preferred, if available), or
  - regular 5V rail (simpler).

If standby power is used:
- define allowed always-on consumption for “sleep” state (target: low single-digit watts).

---
