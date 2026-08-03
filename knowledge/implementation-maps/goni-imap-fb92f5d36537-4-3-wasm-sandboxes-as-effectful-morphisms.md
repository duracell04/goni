---
id: GONI-IMAP-FB92F5D36537
title: 4.3 Wasm sandboxes as effectful morphisms
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: We treat each **tool** or **agent** as a partial function over Arrow objects: $$ T : S \rightsquigarrow T $$ implemented as a Wasm module in an **effectful category** \((\mathcal{A}^\mathsf{eff})\) that extends \((\mathcal{A})\) with side-effects (timers, network, file I/O) via capabilities.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 4.3 Wasm sandboxes as effectful morphisms
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 4.3 Wasm sandboxes as effectful morphisms

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.3 Wasm sandboxes as effectful morphisms

We treat each **tool** or **agent** as a partial function over Arrow objects:
$$
T : S \rightsquigarrow T
$$
implemented as a Wasm module in an **effectful category** \((\mathcal{A}^\mathsf{eff})\) that extends \((\mathcal{A})\) with side-effects (timers, network, file I/O) via capabilities.

We enforce:

> **Invariant E1 (Capability safety).**
> For each sandboxed module \((W)\), there exists a declared capability set \((\mathsf{Cap}(W))\). The host ensures that any effect in \((\mathcal{A}^\mathsf{eff})\) performed by \((W)\) is an element of \((\mathsf{Cap}(W)).

This is enforced by a narrow host API surface (`goni-tool-api`), WASI-like capability handles, and resource limits.
