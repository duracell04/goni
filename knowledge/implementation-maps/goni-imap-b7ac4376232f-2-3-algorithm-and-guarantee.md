---
id: GONI-IMAP-B7AC4376232F
title: 2.3 Algorithm and guarantee
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We implement **lazy greedy**: Initialise \(S_0 = \varnothing\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 2.3 Algorithm and guarantee
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 2.3 Algorithm and guarantee

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.3 Algorithm and guarantee

We implement **lazy greedy**:

1. Initialise \(S_0 = \varnothing\).  
2. At step \(t\), for each \(j \in V \setminus S_t\), compute marginal gain:
   $$
   \Delta(j \mid S_t) = F(S_t \cup \{j\}) - F(S_t).
   $$
3. Choose \(j^\* = \arg\max_j \Delta(j \mid S_t)/c_j\) while \(\sum_{k \in S_t} c_k + c_{j^\*} \le B\).  
4. Set \(S_{t+1} = S_t \cup \{j^\*\}\).  
5. Stop when no item fits.

> **Theorem 2.2 (Approximation bound).**  
> Let \(S^\*\) be an optimal solution and \(S_{\text{greedy}}\) the output of lazy greedy. Then:
> $$
> F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^\*) - \varepsilon
> $$
> for \(\varepsilon\) bounded by numerical precision and the stopping criterion. In practice we target \(\varepsilon \le 10^{-6}\).

> **Invariant C1 (Context guarantee).**  
> For every query,
> $$
> \frac{F(S_{\text{greedy}})}{F(S^\*)} \ge 1 - 1/e - \delta
> $$
> where \(\delta\) is tracked as a runtime statistic and must stay \(< 0.03\) in regression tests.
