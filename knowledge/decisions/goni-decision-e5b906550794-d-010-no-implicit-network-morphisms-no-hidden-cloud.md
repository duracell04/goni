---
id: GONI-DECISION-E5B906550794
title: D-010 – No implicit network morphisms (no hidden cloud)
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** Any effectful morphism involving network I/O is explicitly annotated as such in \(\mathcal{A}^\mathsf{eff}\) and requires configuration.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-010 – No implicit network morphisms (no hidden cloud)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-010 – No implicit network morphisms (no hidden cloud)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-010 – No implicit network morphisms (no hidden cloud)

**Formal statement**

Any effectful morphism involving network I/O is explicitly annotated as such in \(\mathcal{A}^\mathsf{eff}\) and requires configuration.

Formally, for any \(f \in \mathcal{A}^\mathsf{eff}\):
$$
\text{if } \texttt{"network"} \in \mathsf{Effects}(f) \text{ then } f \text{ is opt-in and non-essential}.
$$

**Rationale**

- Aligns with D-001: local-first semantics.  
- Makes it trivial to inspect the code and see where data might leave the machine.

**Consequence**

- Even “harmless” things like version-check pings are explicitly implemented as such and can be disabled.  
- This simplifies compliance and audit (security review can focus on a small number of network-effect morphisms).

---
