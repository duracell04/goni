---
id: GONI-DECISION-5A5B3B8A34CA
title: D-021 - Maximum safe autopilot via autonomy corridors
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Accepted **Date:** 2026-02-23 **Formal statement** Delegable actions are governed by an autonomy policy \((\text{task\_class}, \text{corridor}, \theta)\) where corridor is in \(\{\text{no\_go}, \text{soft\_gate}, \text{autopilot}\}\) and \(\theta\) is a risk threshold.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-021 - Maximum safe autopilot via autonomy corridors
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-021 - Maximum safe autopilot via autonomy corridors

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-021 - Maximum safe autopilot via autonomy corridors

**Status:** Accepted
**Date:** 2026-02-23

**Formal statement**

Delegable actions are governed by an autonomy policy
\((\text{task\_class}, \text{corridor}, \theta)\) where corridor is in
\(\{\text{no\_go}, \text{soft\_gate}, \text{autopilot}\}\) and \(\theta\) is a
risk threshold. Autonomous execution is permitted iff:
$$
\text{corridor} \neq \text{no\_go} \land \text{risk\_score} \le \theta.
$$

**Rationale**

- Aligns product direction with "do as much as possible in the background."
- Keeps autonomy explicit and auditable instead of ad-hoc per-feature toggles.
- Preserves safety by requiring risk-bounded execution and fail-closed fallback.

**Consequence**

- Task classes must declare corridor defaults and thresholds.
- Receipts and audit logs must record autonomy mode and risk basis.
- Conformance includes autonomy throughput and escalation quality metrics.

---
