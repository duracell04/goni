---
id: GONI-IMAP-6114D99A2011
title: 1.3 Relation to linear algebra and relations
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Ignoring nullability and offsets, each column of a batch can be regarded as an element of a finite-dimensional vector space over \(k = \mathbb{R}\) or \(\mathbb{Q}\), and a RecordBatch as an object in \(\mathrm{FinVect}_k\).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 1.3 Relation to linear algebra and relations
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 1.3 Relation to linear algebra and relations

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.3 Relation to linear algebra and relations

Ignoring nullability and offsets, each column of a batch can be regarded as an element of a finite-dimensional vector space over \(k = \mathbb{R}\) or \(\mathbb{Q}\), and a `RecordBatch` as an object in \(\mathrm{FinVect}_k\).

For subsystems with feedback and constraints (e.g. data-dependent scheduling decisions), it is more appropriate to work in the category of **linear relations** \(\mathrm{FinRel}_k\):

- A relation \(R \subseteq U \times V\) models partial, multi-valued transforms.  
- Caps and cups (trace operators) model feedback loops.

At the architecture level we require:

> **Invariant A2 (realizability).**  
> Every linear relation \(R : S \rightsquigarrow T\) used in a dataflow graph must admit an **implementation** as a composite of affine morphisms in \(\mathcal{A}_{rr}^{\text{affine}}\) plus a finite set of **materialisation points** where we explicitly allow allocation.

This lets us draw diagrams in a richer relational calculus, while pinning hot paths to zero-copy implementations.
