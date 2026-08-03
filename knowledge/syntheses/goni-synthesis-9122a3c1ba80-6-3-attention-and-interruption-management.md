---
id: GONI-SYNTHESIS-9122A3C1BA80
title: 6.3 Attention and interruption management
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Interruption decisions should be decision-theoretic: alerts depend on context and predicted interruption cost.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.3 Attention and interruption management
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.3 Attention and interruption management

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.3 Attention and interruption management

Interruption decisions should be decision-theoretic: alerts depend on context
and predicted interruption cost. Models of attention support notify vs defer
decisions. Empirical evidence shows interruption can increase stress and
perceived time pressure, supporting deferral-first policies. [R2-R5, R8]

Goni mapping (normative):
- Interrupt budgets enforce per-channel/per-day caps.
- Deferral first: non-urgent suggestions go to Daily Brief.
- Escalation rule: interrupt only if expected utility of immediate action
  exceeds expected cost of interruption, subject to budget.
