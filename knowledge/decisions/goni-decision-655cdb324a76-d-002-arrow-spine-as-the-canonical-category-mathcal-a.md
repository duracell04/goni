---
id: GONI-DECISION-655CDB324A76
title: D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** All structured internal state is represented as objects of a single category: $$ \mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}} $$ (see 20-architecture) built on Arrow schemas and RecordBatches.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-002 – Arrow Spine as the canonical category \(\mathcal{A}\)

**Formal statement**

All structured internal state is represented as objects of a single category:
$$
\mathcal{A} \equiv \mathcal{A}_{rr}^{\text{affine}}
$$
(see 20-architecture) built on Arrow schemas and `RecordBatch`es.

Any new structured data type MUST:

- Be expressible as a schema \(S \in \mathrm{Ob}(\mathcal{A})\), and  
- Interact with other data via morphisms in \(\mathcal{A}\) or \(\mathcal{A}^\mathsf{eff}\).

**Rationale**

- Replaces a zoo of JSON/SQL/proto formats by a single, columnar algebra.  
- Ensures composability: any pipeline is a morphism in \(\mathcal{A}\) (or its effectful extension).

**Consequence**

- Code that manipulates structured data directly in ad-hoc formats is considered **non-conformant**.  
- For interoperability (e.g. with SQL), we define **functors**:
  $$
  F : \mathcal{A} \to \mathcal{B},
  $$
  e.g. to a relational category, instead of mutating foreign stores directly.

---
