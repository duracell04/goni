---
id: GONI-IMAP-464798632651
title: 1.1 Data Plane \(\mathcal{A}\) – Arrow Spine
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Object.** A symmetric monoidal category $$ \mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}} $$ with: Objects: Arrow schemas \(S\) (finite products of fields).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 1.1 Data Plane \(\mathcal{A}\) – Arrow Spine
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 1.1 Data Plane \(\mathcal{A}\) – Arrow Spine

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1 Data Plane \(\mathcal{A}\) – Arrow Spine

**Object.**  
A symmetric monoidal category
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
with:

- Objects: Arrow schemas \(S\) (finite products of fields).  
- Instances: Arrow `RecordBatch`es of schema \(S\).  
- Morphisms: affine, zero-copy transforms \(f : S \to T\) realised as total functions
  $$
  f^\# : \mathsf{Inst}(S) \to \mathsf{Inst}(T).
  $$

**Invariant A1 (zero-copy).**  
For every hot-path morphism \(f \in \mathcal{A}^{\text{hot}}\) and every batch \(B_S\),
$$
\Delta_\text{alloc}(f^\#, B_S)
= \bigl|\mathsf{Buf}(f^\#(B_S)) \setminus \mathsf{Buf}(B_S)\bigr|
= 0.
$$

**Invariant A2 (affine use).**  
Each payload buffer in \(\mathsf{Buf}(B_S)\) appears at most once in \(\mathsf{Buf}(f^\#(B_S))\); i.e. no “fan-out” of raw buffers on hot paths.

**Proof obligation (theoretical).**

1. Exhibit a set of primitive transforms \(\{f_i\}\) forming generators of \(\mathcal{A}^{\text{hot}}\).  
2. For each \(f_i\), argue from its construction that it reuses or slices only existing buffers.  
3. Show closure: composition and monoidal product of affine morphisms remain affine, hence A1 and A2 hold for all composites used on hot paths.

**Empirical check (MVP).**

- Instrument a small test harness that:
  - Wraps hot-path transforms,  
  - Counts allocations and buffer identities before/after.  
- Property-based test: for random batches up to some size,
  - Assert `?_alloc == 0` and no duplicate payload buffers in outputs.

A node **conforms** on the Data Plane if such tests pass and hot transforms are explicitly enumerated.

---
