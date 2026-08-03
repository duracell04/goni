---
id: GONI-GLOSSARY-CC428029B287
title: 3. Terminology
type: glossary
status: draft
implementation_state: not_applicable
proposition: 'fact: a keyed proposition stored in F_sparse (observed or derived).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 3. Terminology
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 3. Terminology

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Terminology

- fact: a keyed proposition stored in F_sparse (observed or derived).
- goal: a desired state or outcome stored in F_sparse.
- rule: a deterministic transform over symbolic state (facts/goals).
- constraint: a boolean guard that permits or blocks actions or state commits.
- invariant: a constraint that must hold after every commit.
- policy: a kernel-owned set of constraints and authorities.
- proposal: a candidate action or state delta from encoders, agents, or tools.
- validation: symbolic evaluation of a proposal against policy, facts, and
  schemas.
