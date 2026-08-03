---
id: GONI-IMAP-638AE4503025
title: 3.1 Workload model
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We consider a discrete-time queueing model with three classes: Interactive, background, maintenance.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 3.1 Workload model
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3.1 Workload model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Workload model

We consider a discrete-time queueing model with three classes:

- Interactive, background, maintenance.  
- Arrivals \(A_i(t)\) with rates \(\lambda_i\), services with maximum rates \(\mu_i^{\max}\).  
- Queue lengths \(Q_i(t)\) evolving according to:
  $$
  Q_i(t+1) = \max(Q_i(t) - S_i(t), 0) + A_i(t),
  $$
  where \(S_i(t)\) is the service given to class \(i\) at time \(t\).

The **capacity region** is:
$$
\mathcal{C} = \left\{ \boldsymbol{\lambda}\in\mathbb{R}^3_+ :
\sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < 1 \right\}.
$$
