---
id: GONI-SPEC-C6C4D347626D
title: 5.2 Acoustic Behaviour
type: specification
status: draft
implementation_state: specified_only
proposition: "Under light and typical workloads (chat, coding, indexing), the device should be: Subjectively **â€œnear silentâ€\x9D** in a quiet room."
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 5.2 Acoustic Behaviour
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 5.2 Acoustic Behaviour

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.2 Acoustic Behaviour

- Under light and typical workloads (chat, coding, indexing), the device should be:
  - Subjectively **â€œnear silentâ€** in a quiet room.
- Under heavy workloads (adapter training, long-running jobs), fan noise is acceptable but:
  - Should remain within **reasonable desktop/workstation levels**,
  - Should not sound like a â€œserver roomâ€ or high-RPM blower.
