---
id: GONI-SPEC-6C2515DB19BE
title: 3. Default execution policy
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni uses an "auto unless risky" policy: if corridor is autopilot and risk_score <= theta_auto, execute; if corridor is soft_gate and risk_score <= theta_soft, execute with queued review; otherwise escalate to user decision.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 3. Default execution policy
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 3. Default execution policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Default execution policy

Goni uses an "auto unless risky" policy:

- if corridor is `autopilot` and `risk_score <= theta_auto`, execute;
- if corridor is `soft_gate` and `risk_score <= theta_soft`, execute with
  queued review;
- otherwise escalate to user decision.

`theta_auto` and `theta_soft` are explicit policy parameters and must be auditable.
