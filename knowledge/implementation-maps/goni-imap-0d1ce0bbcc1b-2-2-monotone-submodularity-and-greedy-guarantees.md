---
id: GONI-IMAP-0D1CE0BBCC1B
title: 2.2 Monotone submodularity and greedy guarantees
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'It is known that: Each term \(F_i(S) = \max_{j\in S} k(i,j)\) is monotone submodular as a function of \(S\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 2.2 Monotone submodularity and greedy guarantees
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2.2 Monotone submodularity and greedy guarantees

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Monotone submodularity and greedy guarantees

It is known that:

- Each term \(F_i(S) = \max_{j\in S} k(i,j)\) is monotone submodular as a function of \(S\).  
- A non-negative sum of submodular functions is submodular.  
- Adding a non-negative modular term preserves submodularity and monotonicity.

Thus, Goni�s context objective \(F\) is **monotone submodular**. By Nemhauser et al., greedy maximisation under a cardinality or simple knapsack constraint yields:
$$
F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^*),
$$
where \(S^*\) is the optimal set. This gives a clean **approximation guarantee** for each context selection decision.

---
