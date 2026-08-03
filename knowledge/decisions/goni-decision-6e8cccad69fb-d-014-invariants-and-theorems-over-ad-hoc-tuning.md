---
id: GONI-DECISION-6E8CCCAD69FB
title: D-014 – Invariants and theorems over ad-hoc tuning
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** For each major subsystem we choose a set of **invariants** and/or **theorems**, and treat them as part of the public contract: Data Plane: A1, A2.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-014 – Invariants and theorems over ad-hoc tuning
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-014 – Invariants and theorems over ad-hoc tuning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-014 – Invariants and theorems over ad-hoc tuning

**Formal statement**

For each major subsystem we choose a set of **invariants** and/or **theorems**, and treat them as part of the public contract:

- Data Plane: A1, A2.  
- Context Plane: C1.  
- Control Plane: K1, K2.  
- Execution substrate: E1.

We then require that:

1. CI includes tests or simulations that exercise these invariants.  
2. “Optimisations” that would break an invariant are not allowed in stable releases.

**Rationale**

- We want Goni to be a **kernel with proofs**, not just a performing demo.  
- Invariants help future contributors understand what they may and may not change.

**Consequence**

- Some micro-optimisations that improve a single benchmark but violate zero-copy or stability constraints are rejected.  
- Changes to invariants go through an explicit "amendment" process in this document (with versioning and rationale), so we keep a history of our mathematical commitments.

---
