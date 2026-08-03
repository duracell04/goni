---
id: GONI-SPEC-EC048B17CE1E
title: 4. SOP lifecycle
type: specification
status: draft
implementation_state: specified_only
proposition: Standard operating procedures (SOPs) are machine-executable templates.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 4. SOP lifecycle
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 4. SOP lifecycle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. SOP lifecycle

Standard operating procedures (SOPs) are machine-executable templates.
Each SOP MUST move through:

1. `shadow` (suggestions only, no external side effects),
2. `approved` (user approved),
3. `autopilot` (eligible for automatic execution),
4. `revoked` (disabled and retained for audit).

Promotion to `autopilot` requires successful runs and no unresolved safety
findings over a configured observation window.
