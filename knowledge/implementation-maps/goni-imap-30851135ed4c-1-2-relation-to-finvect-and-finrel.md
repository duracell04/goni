---
id: GONI-IMAP-30851135ED4C
title: 1.2 Relation to FinVect and FinRel
type: implementation-map
status: draft
implementation_state: specified_only
proposition: If we ignore nullability and treat fixed-length numeric arrays as vectors over a field \(k\), a RecordBatch can be abstracted as an element of a finite-dimensional vector space \(V\), placing us in \(\mathrm{FinVect}_k\).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 1.2 Relation to FinVect and FinRel
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1.2 Relation to FinVect and FinRel

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2 Relation to FinVect and FinRel

If we ignore nullability and treat fixed-length numeric arrays as vectors over a field \(k\), a `RecordBatch` can be abstracted as an element of a finite-dimensional vector space \(V\), placing us in \(\mathrm{FinVect}_k\).

However, dataflow with constraints, joins, and feedback is more naturally modelled in the category of linear relations \(\mathrm{FinRel}_k\):

- Morphisms are subspaces \(R \subseteq U \times V\), not just linear maps.  
- This allows multi-valued and partially defined transforms.  
- Caps and cups (units and counits) model feedback loops and trace operators.

Goni uses this relational perspective to reason about:

- Joins and filters as relations.  
- Feedback between data and scheduler decisions.  
- The possibility of �wiring diagrams� � la signal-flow calculus.

A design requirement is that every relational diagram used at this level must be **realizable** by an implementation in \(\mathcal{A}_{rr}^{\text{affine}}\) plus a finite set of explicit materialisation points.

---
