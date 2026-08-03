---
id: GONI-IMAP-836D83DB74D7
title: 5. Capability-based execution and local-first semantics
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The Execution Plane is modelled as an **effectful extension** of the Data Plane: Pure data transforms still live in \(\mathcal{A}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 5. Capability-based execution and local-first semantics
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 5. Capability-based execution and local-first semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Capability-based execution and local-first semantics

The Execution Plane is modelled as an **effectful extension** of the Data Plane:

- Pure data transforms still live in \(\mathcal{A}\).  
- Side effects (file I/O, network, time) are wrapped in capabilities and live in \(\mathcal{A}^\mathsf{eff}\).

Each Wasm tool or engine is parameterised by a **capability set** \(\mathsf{Cap}(W)\), and the host enforces:
$$
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W).
$$

The **local-first** requirement is then:

> For the core request?response function \(\mathsf{Run}\), there exists an implementation whose effect trace contains no network capabilities.

This allows us to reason about privacy and sovereignty at the level of the effect system: a conformant local node is simply one where \(\mathsf{Run}\) lives in the **sub-category of local-only effects**.

---
