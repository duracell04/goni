---
id: GONI-EVIDENCE-DECISION-THEORY-01
title: Decision theory separates predictive beliefs from action loss
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Classical statistical decision theory selects actions by minimizing expected loss under a probability model and loss function, supporting a separation between predictive uncertainty and the consequences assigned to possible decisions."
domains:
- research
- evaluation
- policy
aliases: []
relations:
- type: supports
  target: LOSS-01
sources:
- SRC-BERGER1985-STATISTICAL-DECISION-THEORY
artifacts: []
uncertainty: "Classical Bayes decision theory supplies the mathematical framework, while concrete Goni losses and hard policy constraints require domain-specific design and empirical governance."
legacy: []
---

# Decision theory separates predictive beliefs from action loss

Statistical decision theory distinguishes beliefs about uncertain outcomes from
the losses attached to available actions. An action can therefore be rational
under one consequence structure and irrational under another even when the
predictive probabilities are identical.

For Goni, this supports consequence-sensitive routing and escalation. It does
not imply that all governance constraints should be reduced to a scalar utility
function; authority and prohibited actions remain hard policy constraints.
