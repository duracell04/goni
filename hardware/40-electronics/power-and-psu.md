# Power and PSU (MVP draft)

Last refreshed: **2025-12-14**

This document captures the MVP power assumptions, PSU approach, and what must be validated with measurements.

---

## 1. Power budget (APU-centric MVP)

Assumptions (order-of-magnitude):

- APU board sustained: ~120–160 W class (depends on firmware limits and workload)
- Peaks (short): higher bursts are possible
- NVMe SSDs: 5–10 W each under heavy IO (short peaks higher)
- Fans / MCU / misc: 5–15 W

**Design for:** ~250 W sustained worst-case, with headroom for spikes.

---

## 2. PSU decision (MVP)

See ADR-005 in [`../90-decisions.md`](../90-decisions.md).

- Internal PSU: **SFX**, **500–600 W**, **80+ Gold** or better.
- Prefer modern cabling/standards (ATX 3.x) when practical.
- Priority: quiet fan profile at 20–60% load.

Why not a power brick (for MVP):
- brick + DC-DC can be elegant, but adds sourcing complexity and enclosure constraints;
- internal SFX keeps the appliance experience simple for first builds.

---

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

## 4. Validation checklist

Measurements we must run on the reference build:

- Idle power (OS booted, models loaded, no generation)
- Interactive inference power (typical chat)
- Sustained load power (long generation / indexing)
- Peak transient behaviour (spike capture if possible)
- PSU thermals and fan behaviour under each regime
- Wall-to-DC efficiency at typical loads (20–60%)

---

## 5. Risks & mitigations

- **Cable routing in 7 L**: plan harness lengths early; avoid sharp bends.
- **PSU intake recirculation**: duct intake or position vents so PSU doesn’t ingest hot exhaust.
- **Coil whine / acoustic hotspots**: validate multiple PSU SKUs if needed.
