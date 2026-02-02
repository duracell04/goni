# Front Panel MCU (MVP draft)

Last refreshed: **2025-12-14**

This document defines a minimal “front panel controller” so the enclosure has a calm, appliance-like UX.

---

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

## 2. Hardware concept

### MCU choice (pragmatic)

Any small, well-supported MCU with USB device support is fine. Examples:
- RP2040 (USB device, cheap, lots of tooling)
- STM32 (USB FS/HS depending on part)

Key requirement:
- stable USB HID or USB CDC-ACM behaviour on Linux.

### Status bar implementation

Two safe options:

1) **Discrete LED segments** (recommended)
- single-colour LEDs behind a light pipe,
- driven via PWM channels (or an LED driver IC),
- looks “calm” and avoids RGB expectations.

2) Addressable LEDs (only if used as single-colour)
- possible, but introduces RGB optics and supply chain variance.

---

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

## 4. Wiring and connectors

- Button: 2-wire momentary switch
- LED bar: harness to LED PCB/light pipe
- MCU power: 5V (from board USB header or internal hub)
- Grounding: ensure a clean ground reference to avoid noise/flicker

Keep the harness removable and keyed.

---

## 5. Firmware notes (MVP)

- Debounce the button in firmware (and optionally in host).
- Define a “safe default state” if the host is down:
  - e.g. dim “idle” after boot, blink on fault.
- Provide a simple bootloader/update path (UF2 for RP2040 is convenient).

---

## 6. Next steps to make this real

1. Choose MCU family for v1 prototype.
2. Sketch a tiny PCB (or use a dev board for first enclosure prototypes).
3. Implement a minimal host daemon:
   - reads button events,
   - sets state based on system health (service up/down).
