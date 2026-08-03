---
id: GONI-DECISION-1D38615A3B48
title: D-006 – Wasm as the extension substrate (effectful category)
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** Untrusted extensions (tools, agents, connectors) are **not** allowed to define morphisms directly in \(\mathcal{A}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-006 – Wasm as the extension substrate (effectful category)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-006 – Wasm as the extension substrate (effectful category)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-006 – Wasm as the extension substrate (effectful category)

**Formal statement**

Untrusted extensions (tools, agents, connectors) are **not** allowed to define morphisms directly in \(\mathcal{A}\). Instead they live in the effectful category \(\mathcal{A}^\mathsf{eff}\), where:

- Pure data transformations are still morphisms in \(\mathcal{A}\).  
- Side effects (I/O, network) are modelled as morphisms annotated with a capability set.

Let \(W\) be a Wasm module; we associate a capability set \(\mathsf{Cap}(W)\). For any effectful morphism \(f_W \in \mathcal{A}^\mathsf{eff}\) implemented by \(W\):
$$
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W).
$$

**Rationale**

- Makes safety properties (no arbitrary file/network access) explicit in the model.  
- Keeps the core spine (`goni-arrow`) free from unbounded side effects.

**Consequence**

- All extension APIs are mediated by a small, formally specified host interface.  
- Performance-critical extensions must be carefully designed to minimise \(\mathsf{Effects}\) and calls across the sandbox boundary.

---
