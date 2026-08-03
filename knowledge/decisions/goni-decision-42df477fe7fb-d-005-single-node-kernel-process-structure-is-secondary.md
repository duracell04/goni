---
id: GONI-DECISION-42DF477FE7FB
title: D-005 – Single-node kernel, process structure is secondary
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** We treat the node as a single abstract machine implementing: $$ \mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E} $$ regardless of how many OS processes are involved.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-005 – Single-node kernel, process structure is secondary
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-005 – Single-node kernel, process structure is secondary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-005 – Single-node kernel, process structure is secondary

**Formal statement**

We treat the node as a single abstract machine implementing:
$$
\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E}
$$
regardless of how many OS processes are involved.

The initial implementation uses **one process** (`goni-node`) that hosts all four components.

**Rationale**

- Separates logical semantics from OS deployment.  
- Keeps the minimal viable system simple (no distributed consensus, no multi-process scheduling).

**Consequence**

- Later multi-process / multi-node deployments must preserve the same semantics and invariants at the level of \(\mathcal{A},\mathcal{X},\mathcal{K},\mathcal{E}\), treating network boundaries as implementation details, not architectural ones.

---
