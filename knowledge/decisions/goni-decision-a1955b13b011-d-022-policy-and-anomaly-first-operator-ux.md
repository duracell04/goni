---
id: GONI-DECISION-A1955B13B011
title: D-022 - Policy-and-anomaly-first operator UX
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Accepted **Date:** 2026-02-23 **Formal statement** The default operator workflow is: $$ \text{set policy} \to \text{autonomous execution} \to \text{anomaly review}, $$ not per-action approval.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-022 - Policy-and-anomaly-first operator UX
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-022 - Policy-and-anomaly-first operator UX

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-022 - Policy-and-anomaly-first operator UX

**Status:** Accepted
**Date:** 2026-02-23

**Formal statement**

The default operator workflow is:
$$
\text{set policy} \to \text{autonomous execution} \to \text{anomaly review},
$$
not per-action approval.

**Rationale**

- Reduces digital overhead for the owner while preserving meaningful control.
- Improves scaling of autonomy: users govern rules and exceptions, not every
  event.
- Matches HCAI goals (high automation with high user authority).

**Consequence**

- UI surfaces prioritize:
  - policy controls (corridors, thresholds, allow/deny lists),
  - anomaly and drift feeds,
  - batch review and kill-switch actions.
- Feature proposals that rely on repeated per-action confirmations are rejected
  unless no policy-level alternative exists.

---
