---
id: GONI-SYNTHESIS-3DD064A0217F
title: Overrides
type: synthesis
status: draft
implementation_state: specified_only
proposition: Hardware decisions MUST be recorded in blueprint/hardware/90-decisions.md.
domains:
- agent
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/meta/agents.hardware.template.md
  heading: Overrides
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Overrides

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Overrides
- Hardware decisions MUST be recorded in `blueprint/hardware/90-decisions.md`.
- Any numeric hardware claim MUST include: date, "last validated", and an evidence link (benchmark/log/photo).
- Avoid changing reference build assumptions without an ADR update.
