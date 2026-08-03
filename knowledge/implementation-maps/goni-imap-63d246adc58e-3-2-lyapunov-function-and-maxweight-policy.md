---
id: GONI-IMAP-63D246ADC58E
title: 3.2 Lyapunov function and MaxWeight policy
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We fix weights: $$ w_1 = 1,\quad w_2 = 10,\quad w_3 = 100.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.2 Lyapunov function and MaxWeight policy
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.2 Lyapunov function and MaxWeight policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Lyapunov function and MaxWeight policy

We fix weights:
$$
w_1 = 1,\quad w_2 = 10,\quad w_3 = 100.
$$
Define a **quadratic Lyapunov function**:
$$
L(\mathbf{Q}) = Q_1^2 + 100 Q_2^2 + 10000 Q_3^2
= \mathbf{Q}^\top \operatorname{diag}(1,100,10000)\,\mathbf{Q}.
$$

At each decision epoch, we choose which class to serve by **MaxWeight**:

Let \(\mu_i(t)\) be the estimated instantaneous service rate (tokens/s) for class \(i\). Define the pressure:
$$
\Phi_i(\mathbf{Q}(t)) = w_i\,Q_i(t)\,\mu_i(t).
$$

> **Policy K1 (MaxWeight).**  
> Choose
> $$
> i^\*(t) = \arg\max_{i \in \{1,2,3\}} \Phi_i(\mathbf{Q}(t)),
> $$
> and allocate the next quantum of compute (e.g. a token generation step or batching slot) to class \(i^\*\).

This is the policy implemented in `goni-scheduler`.
