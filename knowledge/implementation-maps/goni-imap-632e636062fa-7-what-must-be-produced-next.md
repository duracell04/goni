---
id: GONI-IMAP-632E636062FA
title: 7. What must be produced next
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'To validate feasibility we need: A dimensioned **internal layout drawing** (even a simple SVG) showing: board, PSU, fans/radiator, cable paths.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/30-mechanical/enclosure-notes.md
  heading: 7. What must be produced next
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 7. What must be produced next

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. What must be produced next

To validate feasibility we need:

1. A dimensioned **internal layout drawing** (even a simple SVG) showing:
   - board, PSU, fans/radiator, cable paths.
2. A thermal “budget” note:
   - sustained load assumptions (APU ~120 W class + SSDs + overhead),
   - expected airflow and fan RPM ranges.
3. A prototype plan:
   - cheapest enclosure mock (laser-cut + 3D prints, or modified SFF case) for fit checks.
