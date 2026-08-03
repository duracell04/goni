---
id: GONI-PRINCIPLE-577BB90DD512
title: 8. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**I1 - Policy primacy:** no autonomous execution without an explicit corridor and policy hash.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 8. Invariants
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 8. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Invariants

- **I1 - Policy primacy:** no autonomous execution without an explicit corridor
  and policy hash.
- **I2 - Risk-bounded autonomy:** autonomous execution requires computed risk
  below active thresholds.
- **I3 - Auditable delegation:** every autonomous or escalated action emits a
  receipt with autonomy and risk fields.
- **I4 - Fail closed:** if risk computation, policy load, or capability
  validation fails, execution is denied and logged.
- **I4a - Kernel-owned corridors:** authority corridors are defined and applied
  by kernel policy, not by third-party session or gateway logic.
- **I5 - Visible intent repair:** mutating execution requires an auditable chain
  from repaired intent to plan to tool intent.
- **I6 - Decisive questioning only:** clarification interrupts are allowed only
  when they materially change safe execution or policy outcome.
- **I7 - Surfaced assumptions:** proceeding under ambiguity requires explicit
  assumption and uncertainty metadata.
- **I8 - No silent goal selection:** genuine goal ambiguity must remain visible
  through `co_creation` handling or blocking.
