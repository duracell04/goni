---
id: GONI-IMAP-086FADF076BA
title: 1.2 Context Plane \(\mathcal{X}\) – Submodular Context Selection
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Object.** A constrained monotone submodular maximisation problem over a ground set \(V\): Ground set \(V\) of chunks (top-K retrieved).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 1.2 Context Plane \(\mathcal{X}\) – Submodular Context Selection
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 1.2 Context Plane \(\mathcal{X}\) – Submodular Context Selection

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2 Context Plane \(\mathcal{X}\) – Submodular Context Selection

**Object.**  
A constrained monotone submodular maximisation problem over a ground set \(V\):

- Ground set \(V\) of chunks (top-K retrieved).  
- Cost \(c_i \in \mathbb{N}\) (token length) for each \(i \in V\).  
- Embeddings \(e_i \in \mathbb{R}^d\), query relevance scores \(r_i \ge 0\).  
- Objective:
  $$
  F(S) =
  \sum_{i \in V} \max_{j \in S} \cos(e_i, e_j)
  + \gamma \sum_{j \in S} r_j
  $$
- Constraint:
  $$
  \sum_{j \in S} c_j \le B.
  $$

**Invariant C1 (submodularity and guarantee).**

1. \(F\) is **monotone submodular** on \(2^V\).  
2. The implemented selector \(\mathsf{Select}\) uses greedy (or lazy greedy) and satisfies:
   $$
   F(S_{\text{greedy}}) \ge (1 - 1/e)\,F(S^\*) - \varepsilon
   $$
   with \(\varepsilon\) controlled (target \(\varepsilon \le 10^{-6}\)).

**Proof obligation (theoretical).**

- Show that a single facility-location term \(\sum_{i} \max_{j\in S} k(i,j)\) with a non-negative similarity kernel \(k\) is monotone submodular.  
- Show that adding a non-negative modular term \(\gamma \sum_{j\in S} r_j\) preserves monotone submodularity.  
- Cite or reproduce the Nemhauser–Wolsey bound for greedy on monotone submodular maximisation under a knapsack or cardinality constraint.

**Empirical check (MVP).**

- On small problems (e.g. \(|V| \le 10\)):
  - Compute the exact optimum \(S^\*\) by brute force.  
  - Compute greedy solution \(S_{\text{greedy}}\).  
  - Assert
    $$
    F(S_{\text{greedy}}) \ge 0.63\, F(S^\*).
    $$
- Track ratio \(F_{\text{greedy}} / F_{\text{opt}}\) distribution across random synthetic instances.

A node **conforms** on the Context Plane if:

- The implemented objective is monotone submodular, and  
- Greedy meets (or empirically approximates) the \((1-1/e)\) bound on small instances.

---
