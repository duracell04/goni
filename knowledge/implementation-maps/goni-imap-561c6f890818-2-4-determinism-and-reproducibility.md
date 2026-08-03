---
id: GONI-IMAP-561C6F890818
title: 2.4 Determinism and reproducibility
type: implementation-map
status: draft
implementation_state: specified_only
proposition: For a fixed snapshot of the Data Plane (fixed embeddings, fixed ANN retrieval order), the context selection must be **deterministic**.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 2.4 Determinism and reproducibility
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 2.4 Determinism and reproducibility

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.4 Determinism and reproducibility

For a fixed snapshot of the Data Plane (fixed embeddings, fixed ANN retrieval order), the context selection must be **deterministic**.

Formally, there exists a pure function
$$
\mathsf{Select} : (q, V, B) \mapsto S \subseteq V
$$
such that repeated calls with the same arguments yield the same selected set \(S\).

Implementation constraints:

- `goni-context` uses only deterministic operations; random tiebreakers are derived from stable chunk IDs.  
- Budget and similarity calculations are pure functions of inputs.
