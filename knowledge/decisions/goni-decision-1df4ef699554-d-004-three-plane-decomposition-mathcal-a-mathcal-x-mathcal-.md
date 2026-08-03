---
id: GONI-DECISION-1DF4EF699554
title: D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** The semantics of a node are factored as: $$ \mathsf{Run} = F_{\mathcal{E}} \circ F_{\mathcal{K}} \circ F_{\mathcal{X}} \circ F_{\mathcal{A}} $$ with: \(F_{\mathcal{A}}\) – retrieves/manipulates data purely via \(\mathcal{A}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-004 – Three-plane decomposition (\(\mathcal{A},\mathcal{X},\mathcal{K}\))

**Formal statement**

The semantics of a node are factored as:
$$
\mathsf{Run} = F_{\mathcal{E}} \circ F_{\mathcal{K}} \circ F_{\mathcal{X}} \circ F_{\mathcal{A}}
$$
with:

- \(F_{\mathcal{A}}\) – retrieves/manipulates data purely via \(\mathcal{A}\).  
- \(F_{\mathcal{X}}\) – solves a submodular optimisation problem over retrieved chunks.  
- \(F_{\mathcal{K}}\) – schedules and routes work using \(\mathcal{K}\).  
- \(F_{\mathcal{E}}\) – executes engines / tools (\(\mathcal{E}\)).

**Rationale**

- Ensures there is **no direct path** from raw connectors to engines.  
- Each plane can have its own local invariants and be tested in isolation.

**Consequence**

- Architectural reviews must ask: “Which plane does this feature belong to?”  
- “Shortcut” code that calls models directly out of a connector is rejected by design.

---
