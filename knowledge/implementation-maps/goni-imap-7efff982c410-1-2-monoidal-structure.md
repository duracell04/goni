---
id: GONI-IMAP-7EFFF982C410
title: 1.2 Monoidal structure
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The monoidal product of \(\mathcal{A}\) is **schema concatenation**: On objects: $$ S \oplus T = (f_1:\tau_1, \dots, f_m:\tau_m, g_1:\sigma_1, \dots, g_k:\sigma_k) $$ On instances: $$ B_S \otimes B_T \coloneqq \texttt{RecordBatch::try\_new}(S\oplus T, [a_1,\dots,a_m,b_1,\dots,b_k]) $$ which is again zero-copy because we only re-use ArrayData handles.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 1.2 Monoidal structure
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 1.2 Monoidal structure

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2 Monoidal structure

The monoidal product of \(\mathcal{A}\) is **schema concatenation**:

- On objects:
  $$
  S \oplus T = (f_1:\tau_1, \dots, f_m:\tau_m, g_1:\sigma_1, \dots, g_k:\sigma_k)
  $$
- On instances:
  $$
  B_S \otimes B_T \coloneqq \texttt{RecordBatch::try\_new}(S\oplus T, [a_1,\dots,a_m,b_1,\dots,b_k])
  $$
  which is again zero-copy because we only re-use `ArrayData` handles.

> **Invariant A1 (monoidal zero-copy).**  
> For all \(f, g \in \mathcal{A}_{rr}^{\text{affine}}\),
> $$
> \Delta_\text{alloc}(f \otimes g,\, B_S \otimes B_T) = 0.
> $$

The unit object is the empty schema \(I \equiv ()\), represented as `Schema::empty()`.
