---
id: GONI-IMAP-55620E7806AF
title: '1.1 Objects: schemas, instances, and `RecordBatch`'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: We model the Data Plane as a symmetric monoidal category $$ \mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}, $$ whose objects are Arrow schemas and whose morphisms are affine, zero-copy transforms.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: '1.1 Objects: schemas, instances, and `RecordBatch`'
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1.1 Objects: schemas, instances, and `RecordBatch`

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1 Objects: schemas, instances, and `RecordBatch`

We model the Data Plane as a symmetric monoidal category
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}},
$$
whose objects are Arrow schemas and whose morphisms are affine, zero-copy transforms.

- Each schema \(S\) corresponds to an object.  
- Instances \(B_S\) correspond to generalised �states� of that object.  
- A transform \(f : S \to T\) is a morphism \(f^\#: \mathsf{Inst}(S) \to \mathsf{Inst}(T)\) respecting the affine/zero-copy constraints.

The monoidal product \(\oplus\) is **schema concatenation**, corresponding physically to `RecordBatch::try_new` with multiple columns sharing underlying buffers.
