---
id: GONI-IMAP-93BF5C267F22
title: 2.2 Objective and optimisation problem
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We define the **facility location + relevance** objective: $$ F(S) = \underbrace{ \sum_{i \in V} \max_{j \in S} \cos(e_i, e_j) }_{\text{coverage term}} + \gamma \underbrace{\sum_{j \in S} r_j}_{\text{relevance term}} \quad \text{for } S \subseteq V, $$ with trade-off parameter \(\gamma \ge 0\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 2.2 Objective and optimisation problem
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 2.2 Objective and optimisation problem

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Objective and optimisation problem

We define the **facility location + relevance** objective:
$$
F(S) =
\underbrace{
\sum_{i \in V} \max_{j \in S} \cos(e_i, e_j)
}_{\text{coverage term}} +
\gamma \underbrace{\sum_{j \in S} r_j}_{\text{relevance term}}
\quad \text{for } S \subseteq V,
$$
with trade-off parameter \(\gamma \ge 0\).

This induces a constrained maximisation problem:
$$
\max_{S \subseteq V}
\quad F(S)
\quad \text{s.t.} \quad \sum_{j \in S} c_j \le B.
$$

> **Proposition 2.1.**  
> The function \(F : 2^V \to \mathbb{R}_{\ge 0}\) is **monotone submodular**.  
> (Monotone: \(F(S) \le F(T)\) when \(S \subseteq T\). Submodular: diminishing returns.)
