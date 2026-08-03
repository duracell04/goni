---
id: GONI-PRINCIPLE-9A89D2BA0617
title: 7. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: ITCR is interrupt-driven and admission-controlled (SCHED-01).
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 7. Invariants
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 7. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Invariants

- ITCR is interrupt-driven and admission-controlled (SCHED-01).
- No ITCR output bypasses SS-01 validation or TOOL-01 envelopes.
- All ITCR activity is auditable and tied to a state snapshot.
- Budgets are enforced for time, search, and tool-planning depth.
