---
id: GONI-DECISION-7CD481DA15B7
title: D-001 – Local-first, offline-capable by definition
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** A node \(N\) is *valid* iff the function $$ \mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log} $$ is total and computable **using only local state and local compute**.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-001 – Local-first, offline-capable by definition
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-001 – Local-first, offline-capable by definition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-001 – Local-first, offline-capable by definition

**Formal statement**

A node \(N\) is *valid* iff the function
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$
is total and computable **using only local state and local compute**.

Equivalently: in the category of effectful computations \(\mathcal{A}^\mathsf{eff}\), any morphism used by the kernel must have an implementation that does not depend on remote network calls.

**Rationale**

- Eliminates remote-service availability as a factor in correctness.  
- Makes privacy and data-sovereignty constraints natural: all state lives in local objects of \(\mathcal{A}\).

**Consequence**

Any network effect \(e \in \mathsf{Effect}(\mathcal{A}^\mathsf{eff})\) is:

- Either outside the kernel (connectors, opt-in sync),  
- Or explicitly marked as “non-essential” (failure does not break \(\mathsf{Run}\) for local requests).

---
