---
id: GONI-DECISION-E18EFE88D5EB
title: D-003 – Affine, zero-copy morphisms in hot paths
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** We distinguish: \(\mathcal{A}^{\text{hot}}\): morphisms used on hot paths (per-request data handling).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-003 – Affine, zero-copy morphisms in hot paths
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-003 – Affine, zero-copy morphisms in hot paths

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-003 – Affine, zero-copy morphisms in hot paths

**Formal statement**

We distinguish:

- \(\mathcal{A}^{\text{hot}}\): morphisms used on hot paths (per-request data handling).  
- \(\mathcal{A}^{\text{cold}}\): morphisms off hot paths (debug, export).

Constraint:
$$
\mathcal{A}^{\text{hot}} \subseteq \mathcal{A}_{rr}^{\text{affine}}.
$$

That is, for all \(f \in \mathcal{A}^{\text{hot}}\),
$$
\Delta_\text{alloc}(f^\#, B_S) = 0
\quad\forall B_S.
$$

**Rationale**

- Keeps memory and cache behaviour predictable.  
- Enables compositional reasoning: composition of hot-path transforms stays zero-copy.

**Consequence**

- When defining new transforms, we must categorise them as `hot` or `cold`.  
- CI includes property-based tests that check `hot` transforms allocate **no payload buffers**.

---
