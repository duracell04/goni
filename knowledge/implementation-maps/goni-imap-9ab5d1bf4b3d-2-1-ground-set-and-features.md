---
id: GONI-IMAP-9AB5D1BF4B3D
title: 2.1 Ground set and features
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Let: \(V\) be the set of candidate chunks for a query (top-\(K\) from ANN, typically \(K=512\)).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 2.1 Ground set and features
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 2.1 Ground set and features

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Ground set and features

Let:

- \(V\) be the set of candidate chunks for a query (top-\(K\) from ANN, typically \(K=512\)).  
- For each \(i \in V\), we store:
  - embedding \(e_i \in \mathbb{R}^d\) (normalised, \(d = 1024\)),  
  - cost \(c_i \in \mathbb{N}\) (token length),  
  - query relevance score \(r_i \in \mathbb{R}_{\ge 0}\) (e.g. cosine similarity to query embedding \(q\)).

Let \(B\) be a token budget (prompt + context).
