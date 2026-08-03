---
id: GONI-IMAP-DB5BF1E2F968
title: 1.1 Objects and morphisms
type: implementation-map
status: draft
implementation_state: specified_only
proposition: We define a symmetric monoidal category $$ \mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}} $$ whose objects are Arrow schemas and whose morphisms are **affine, zero-copy transforms** between Arrow RecordBatches.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 1.1 Objects and morphisms
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 1.1 Objects and morphisms

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1 Objects and morphisms

We define a symmetric monoidal category
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
whose objects are Arrow schemas and whose morphisms are **affine, zero-copy transforms** between Arrow `RecordBatch`es.

- **Objects.**  
  An object \(S \in \mathrm{Ob}(\mathcal{A})\) is a finite, ordered product of Arrow fields:
  $$
  S = (f_1 : \tau_1, \dots, f_n : \tau_n)
  $$
  and is represented in code as `SchemaRef`.

- **Instances.**  
  For each schema \(S\), the set of instances \(\mathsf{Inst}(S)\) is the set of Arrow `RecordBatch`es whose schema is \(S\).  
  In practice, an instance is a tuple of `ArrayData` values
  $$
  B_S = (a_1, \dots, a_n), \quad a_i \in \text{ArrayData}(\tau_i)
  $$
  each backed by one or more buffers \(b \in \text{Buffer} \cong \text{Arc<[u8]>}\).

- **Morphisms.**  
  A morphism \(f : S \to T\) is implemented as a total function:
  $$
  f^\# : \mathsf{Inst}(S) \to \mathsf{Inst}(T)
  $$
  such that:

  1. (**Affine use**) each input buffer is used in constructing at most one output buffer; we never â€œfan-outâ€ raw buffers.  
  2. (**Zero-copy**) any new `ArrayData` is built exclusively from:
     - existing buffers via slice (`offset`, `len`), or  
     - new **metadata only** (offsets/lengths, validity bitmaps) but **no new payload buffers**.

Formally, let \(\mathsf{Buf}(B)\) be the multiset of payload buffers of a batch \(B\). Define
$$
\Delta_\text{alloc}(f^\#, B_S) \equiv \bigl|\mathsf{Buf}(f^\#(B_S)) \setminus \mathsf{Buf}(B_S)\bigr|
$$
(counting only newly allocated payload buffers, not metadata).

> **Definition 1 (Affine zero-copy morphism).**  
> A morphism \(f^\#: \mathsf{Inst}(S) \to \mathsf{Inst}(T)\) is in \(\mathcal{A}_{rr}^{\text{affine}}\) iff, for all inputs \(B_S\),
> $$
> \Delta_\text{alloc}(f^\#, B_S) = 0
> \quad\text{and}\quad
> \text{each } b \in \mathsf{Buf}(B_S) \text{ appears at most once in } \mathsf{Buf}(f^\#(B_S)).
> $$

This is enforced in code by constraining transforms to:

- borrow slices (`ArrayData::new` with existing `Arc<Buffer>`), and  
- reject transforms that construct new payload `Buffer`s on hot paths.
