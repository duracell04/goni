# Enclosure Notes (MVP draft)

Last refreshed: **2025-12-14**

This is a draft mechanical plan for validating feasibility of the MVP architecture (APU-centric, ~7 L).

---

## 1. Envelope targets

- **Target volume:** ~7 L (allowed range 6–8 L).
- **Form:** clean rectangular appliance, minimal front.
- **Orientation:** upright “book” orientation preferred (smaller footprint), but horizontal is acceptable if thermals are better.

Recommended starting point (to iterate from):
- External dimensions in the ballpark of **~210 × 210 × 160 mm** (example only; adjust after internal layout).

---

## 2. Internal layout (first-pass)

Core components to fit:

- Mini-ITX-style APU mainboard (Framework Desktop mainboard is reference)
- Internal SFX PSU (500–600 W)
- 2× M.2 NVMe (on-board + secondary slot; add heatsinks)
- Cooling:
  - either a 240 mm AIO radiator (if layout supports it cleanly),
  - or a high-end air tower + 2–3× 120 mm PWM fans.

### Candidate layout A (air-cooled, simplest)

- Board mounted to one side panel (standoffs).
- Large tower cooler oriented to push air front → back (or bottom → top).
- One 120/140 mm intake + one 120/140 mm exhaust (low RPM).
- PSU isolated intake path (separate vent) to avoid PSU recycling hot air.

Why: fewer failure modes than AIO; easier service; lower BOM risk.

### Candidate layout B (240 mm AIO, quiet-at-load)

- 240 mm radiator mounted to one face (side or top), with 2× 120 mm fans at low RPM.
- Board mounted opposite radiator.
- PSU in its own compartment or with a ducted intake.

Why: tends to keep the APU cooler under sustained load without ramping fans.

---

## 3. Airflow & dust

- Aim for **large, low-RPM fans** and **straight flow paths**.
- Use **removable dust filters** on all intakes.
- Ensure no “hot spot recirculation loops”:
  - PSU should not ingest APU exhaust,
  - NVMe should have at least some directed airflow.

---

## 4. Acoustics strategy

- “Quiet by default”: tune fan curves for interactive workloads.
- Prefer:
  - 120/140 mm fans,
  - rubber grommets for fan mounts,
  - minimal panel resonance (stiff panels, proper fasteners).
- Avoid:
  - small blowers,
  - mesh that whistles at mid RPM.

---

## 5. Serviceability & assembly

- Tool-less access is nice, but MVP can start with “4 screws”.
- Minimum serviceability requirements:
  - NVMe access without full disassembly,
  - PSU replacement without removing the board,
  - front-panel harness accessible.

---

## 6. External I/O & front panel

Back panel:
- power inlet (IEC C14 or PSU-specific),
- at least 1× RJ45 (5 GbE preferred),
- USB ports and display outputs as provided by the compute module.

Front panel (MVP):
- single power button
- calm status bar (single-colour segments; no RGB effects)
- optional: small pinhole for “reset/service” (hidden)

---

## 7. What must be produced next

To validate feasibility we need:

1. A dimensioned **internal layout drawing** (even a simple SVG) showing:
   - board, PSU, fans/radiator, cable paths.
2. A thermal “budget” note:
   - sustained load assumptions (APU ~120 W class + SSDs + overhead),
   - expected airflow and fan RPM ranges.
3. A prototype plan:
   - cheapest enclosure mock (laser-cut + 3D prints, or modified SFF case) for fit checks.
