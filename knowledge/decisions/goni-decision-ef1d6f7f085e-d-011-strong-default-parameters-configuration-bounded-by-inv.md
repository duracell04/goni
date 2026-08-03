---
id: GONI-DECISION-EF1D6F7F085E
title: D-011 – Strong default parameters; configuration bounded by invariants
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** We treat all tunable parameters (e.g.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-011 – Strong default parameters; configuration bounded by invariants
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-011 – Strong default parameters; configuration bounded by invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-011 – Strong default parameters; configuration bounded by invariants

**Formal statement**

We treat all tunable parameters (e.g. \(\gamma\), \(B\), scheduler weights, router thresholds) as living inside **safe regions** defined by invariants.

Example:

- Context plane: choose \(\gamma\) and \(B\) such that C1 holds and prompt budgets per model are respected.  
- Control plane: choose admission thresholds so that K1 holds.

Parameters outside these safe regions are allowed only in “experimental” modes.

**Rationale**

- Keeps default nodes in the regime where our theorems apply.  
- Makes configuration safer: users can change things without accidentally destroying stability.

**Consequence**

- Config parsing includes validation against invariant ranges.  
- Documentation describes both: (a) default values, (b) safe ranges implied by theory.

---
