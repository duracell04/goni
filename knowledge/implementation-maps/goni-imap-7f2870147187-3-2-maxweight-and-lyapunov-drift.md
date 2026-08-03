---
id: GONI-IMAP-7F2870147187
title: 3.2 MaxWeight and Lyapunov drift
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We define a quadratic Lyapunov function: $$ L(\mathbf{Q}) = \mathbf{Q}^\top D\,\mathbf{Q},\quad D = \operatorname{diag}(1, 100, 10000), $$ assigning different penalties to different classes.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 3.2 MaxWeight and Lyapunov drift
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3.2 MaxWeight and Lyapunov drift

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 MaxWeight and Lyapunov drift

We define a quadratic Lyapunov function:
$$
L(\mathbf{Q}) = \mathbf{Q}^\top D\,\mathbf{Q},\quad
D = \operatorname{diag}(1, 100, 10000),
$$
assigning different penalties to different classes.

The **MaxWeight** policy chooses at each time:
$$
i^*(t) = \arg\max_i \, w_i\,Q_i(t)\,\mu_i(t),
$$
with weights \(w_i\) aligned with the diagonal of \(D\).

Standard results in stochastic network theory show that, under mild conditions (e.g. i.i.d. arrivals, bounded increments), MaxWeight stabilises any arrival vector in the interior of \(\mathcal{C}\):

- The expected one-step Lyapunov drift is negative outside a compact set.  
- This implies positive recurrence and finite expected queue lengths.

Goni leverages this to claim: **if we configure utilisation below the boundary of \(\mathcal{C}\)** (with a safety margin \(\alpha < 1\)), and use a MaxWeight-like scheduler, then interactive queues remain stable even under mixed workloads.

---
