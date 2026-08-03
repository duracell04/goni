---
id: GONI-IMAP-776AC34248D5
title: MCU choice (pragmatic)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Any small, well-supported MCU with USB device support is fine.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: MCU choice (pragmatic)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# MCU choice (pragmatic)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### MCU choice (pragmatic)

Any small, well-supported MCU with USB device support is fine. Examples:
- RP2040 (USB device, cheap, lots of tooling)
- STM32 (USB FS/HS depending on part)

Key requirement:
- stable USB HID or USB CDC-ACM behaviour on Linux.
