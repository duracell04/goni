---
id: GONI-IMAP-85FF7037F532
title: 6. Implementation overview (code â†” math)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '| Plane / Object | Formal notion | Main Rust crates | | \((\mathcal{A})\) | \((\mathcal{A}_{rr}^{\text{affine}})\) | goni-arrow, goni-store, goni-index | | \((\mathcal{X})\) | submodular optimisation over \((2^V)\) | goni-context, goni-prompt |'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 6. Implementation overview (code â†” math)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 6. Implementation overview (code â†” math)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Implementation overview (code â†” math)

| Plane / Object | Formal notion                                      | Main Rust crates                               |
| -------------- | -------------------------------------------------- | ---------------------------------------------- |
| \((\mathcal{A})\)  | \((\mathcal{A}_{rr}^{\text{affine}})\)                 | `goni-arrow`, `goni-store`, `goni-index`       |
| \((\mathcal{X})\)  | submodular optimisation over \((2^V)\)                 | `goni-context`, `goni-prompt`                  |
| \((\mathcal{K})\)  | queueing network + Lyapunov scheduler, router      | `goni-scheduler`, `goni-router`, `goni-resman` |
| \((\mathcal{E})\)  | engines and sandboxes (\((\mathcal{A}^\mathsf{eff}))\) | `goni-engine-*`, `goni-wasm`, `goni-tool-api`  |

All future contributions should be expressible as:

* new objects / morphisms in \((\mathcal{A}),
* new objective terms or constraints in \((\mathcal{X}),
* new classes or policies in \((\mathcal{K}), or
* new engines / sandboxes in \((\mathcal{E}),

without breaking the stated invariants.
