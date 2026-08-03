---
id: GONI-IMAP-27DB4ED63C95
title: Candidate layout A (air-cooled, simplest)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Board mounted to one side panel (standoffs).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/30-mechanical/enclosure-notes.md
  heading: Candidate layout A (air-cooled, simplest)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Candidate layout A (air-cooled, simplest)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Candidate layout A (air-cooled, simplest)

- Board mounted to one side panel (standoffs).
- Large tower cooler oriented to push air front → back (or bottom → top).
- One 120/140 mm intake + one 120/140 mm exhaust (low RPM).
- PSU isolated intake path (separate vent) to avoid PSU recycling hot air.

Why: fewer failure modes than AIO; easier service; lower BOM risk.
