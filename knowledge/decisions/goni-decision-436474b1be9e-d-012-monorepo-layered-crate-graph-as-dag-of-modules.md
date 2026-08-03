---
id: GONI-DECISION-436474B1BE9E
title: D-012 – Monorepo + layered crate graph as DAG of modules
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** The core project is a monorepo whose crate dependency graph is a **directed acyclic graph (DAG)**: There exists a partial order \(\prec\) on crates such that if crate \(A\) depends on crate \(B\), then \(B \prec A\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-012 – Monorepo + layered crate graph as DAG of modules
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-012 – Monorepo + layered crate graph as DAG of modules

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-012 – Monorepo + layered crate graph as DAG of modules

**Formal statement**

The core project is a monorepo whose crate dependency graph is a **directed acyclic graph (DAG)**:

- There exists a partial order \(\prec\) on crates such that if crate \(A\) depends on crate \(B\), then \(B \prec A\).  
- “Lower” crates are closer to the math (Arrow, scheduler); “higher” crates implement user-facing behaviour.

**Rationale**

- Acyclic graph reflects the mathematical layering: \(\mathcal{A} \to \mathcal{X} \to \mathcal{K} \to \mathcal{E} \to \text{UI}\).  
- Simplifies reasoning about where invariants are enforced (at the bottom of the DAG).

**Consequence**

- Introducing a dependency cycle is considered a structural bug; CI rejects it.  
- Cross-cutting functionality (e.g. tracing) must be injected via interfaces, not by making everything depend on everything.

---
