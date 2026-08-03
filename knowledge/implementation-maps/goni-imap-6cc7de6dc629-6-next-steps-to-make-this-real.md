---
id: GONI-IMAP-6CC7DE6DC629
title: 6. Next steps to make this real
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Choose MCU family for v1 prototype.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/front-panel-mcu.md
  heading: 6. Next steps to make this real
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 6. Next steps to make this real

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Next steps to make this real

1. Choose MCU family for v1 prototype.
2. Sketch a tiny PCB (or use a dev board for first enclosure prototypes).
3. Implement a minimal host daemon:
   - reads button events,
   - sets state based on system health (service up/down).
