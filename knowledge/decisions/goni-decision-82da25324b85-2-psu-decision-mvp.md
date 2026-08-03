---
id: GONI-DECISION-82DA25324B85
title: 2. PSU decision (MVP)
type: decision
status: draft
implementation_state: specified_only
proposition: See ADR-005 in ../90-decisions.md.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/power-and-psu.md
  heading: 2. PSU decision (MVP)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. PSU decision (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. PSU decision (MVP)

See ADR-005 in [`../90-decisions.md`](/blueprint/hardware/90-decisions.md).

- Internal PSU: **SFX**, **500–600 W**, **80+ Gold** or better.
- Prefer modern cabling/standards (ATX 3.x) when practical.
- Priority: quiet fan profile at 20–60% load.

Why not a power brick (for MVP):
- brick + DC-DC can be elegant, but adds sourcing complexity and enclosure constraints;
- internal SFX keeps the appliance experience simple for first builds.

---
