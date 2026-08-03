---
id: GONI-IMAP-367B2484134A
title: 5. End-to-end semantics
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'At the highest level, a **request** is an element of a type: $$ \mathsf{Req} = (\text{user_msg}, \text{tools}, \text{profile}, \text{budgets}, \dots) $$ and a **response stream** is a sequence of tokens plus logs and tool results.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 5. End-to-end semantics
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 5. End-to-end semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. End-to-end semantics

At the highest level, a **request** is an element of a type:
$$
\mathsf{Req} = (\text{user_msg}, \text{tools}, \text{profile}, \text{budgets}, \dots)
$$
and a **response stream** is a sequence of tokens plus logs and tool results.

We can regard the node as computing a (possibly stochastic) function:
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$

Implementation-wise, this is the composition of:

1. Parsing / admission: \((\mathsf{Req} \to \text{job in class } i)\).
2. Data Plane retrieval: functors in \((\mathcal{A}).
3. Context selection: optimisation in \((\mathcal{X})\) with guarantee C1.
4. Scheduling + routing: \((\mathcal{K})\) with invariants K1, K2.
5. Engine execution: morphisms in \((\mathcal{E}).
6. Logging: \((\mathcal{A})\) again (metrics as Arrow tables).

Our **architectural contract** is that each piece satisfies its invariants; the composition then has predictable boundaries on resource use, latency and information quality.

---
