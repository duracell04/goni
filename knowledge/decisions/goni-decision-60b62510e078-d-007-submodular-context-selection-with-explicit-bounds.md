---
id: GONI-DECISION-60B62510E078
title: D-007 – Submodular context selection with explicit bounds
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** Context selection is always expressed as: $$ \max_{S \subseteq V} F(S) \quad \text{s.t.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-007 – Submodular context selection with explicit bounds
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-007 – Submodular context selection with explicit bounds

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-007 – Submodular context selection with explicit bounds

**Formal statement**

Context selection is always expressed as:
$$
\max_{S \subseteq V} F(S) \quad \text{s.t. } \sum_{i \in S} c_i \le B
$$
where \(F\) is **monotone submodular**, and solved using a greedy (or accelerated greedy) algorithm with known approximation guarantees (Theorem 2.2).

**Rationale**

- Gives a **lower bound** on the quality of the context we provide to models.  
- Provides a clear knob (\(\gamma\), \(B\)) for tuning diversity vs relevance.

**Consequence**

- Any proposal to change context selection must either:
  - Define a new submodular \(F'\) and maintain a similar bound, or  
  - Explicitly justify why we abandon mathematical guarantees.

---
