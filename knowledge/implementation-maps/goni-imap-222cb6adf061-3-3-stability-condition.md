---
id: GONI-IMAP-222CB6ADF061
title: 3.3 Stability condition
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We assume the **capacity region**: $$ \mathcal{C} = \left\{ \boldsymbol{\lambda} \in \mathbb{R}_+^3 : \sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < 1 \right\}.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.3 Stability condition
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.3 Stability condition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Stability condition

We assume the **capacity region**:
$$
\mathcal{C} = \left\{ \boldsymbol{\lambda} \in \mathbb{R}_+^3 : \sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < 1 \right\}.
$$

We operationalise this with a safety factor \(\alpha \in (0,1)\), and configure admission control so that:
$$
\sum_{i=1}^3 \frac{\lambda_i}{\mu_i^{\max}} < \alpha
\quad\text{with }\alpha = 0.94\text{ by default.}
$$

> **Theorem 3.1 (Queue stability, fluid limit).**  
> Under mild assumptions on arrivals (e.g. ergodic, bounded second moments), if the arrival rate vector \(\boldsymbol{\lambda}\) lies in the interior of \(\alpha \mathcal{C}\) with \(\alpha < 1\), then the MaxWeight policy K1 stabilises the network in the sense that:
> $$
> \sup_{t \ge 0} \mathbb{E}[L(\mathbf{Q}(t))] < \infty,
> $$
> and the fluid limits of \(\mathbf{Q}(t)\) converge to 0.

> **Invariant K1 (Configured stability).**  
> The node enforces a token-budget admission control policy such that the empirical estimate of \(\boldsymbol{\lambda}\) satisfies:
> $$
> \sum_{i=1}^3 \frac{\hat{\lambda}_i}{\mu_i^{\max}} \le 0.94.
> $$
> Simulation tests must show \(\mathbb{E}[L(\mathbf{Q}(t))]\) remains bounded over long horizons under representative workloads.
