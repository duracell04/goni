---
id: GONI-SPEC-6DF194C8D115
title: 3.3 Power Envelope
type: specification
status: draft
implementation_state: specified_only
proposition: The system should operate comfortably from a **standard household power outlet**.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.3 Power Envelope
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.3 Power Envelope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Power Envelope

- The system should operate comfortably from a **standard household power outlet**.
- Target sustained draw under heavy AI workloads: **on the order of a few hundred watts**, not kilowatts.
- Peak draw should be handled by the power subsystem without instability or audible stress.
