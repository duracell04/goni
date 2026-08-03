---
id: GONI-SPEC-69621F33573E
title: 3.3 Filesystem mutation path
type: specification
status: draft
implementation_state: specified_only
proposition: 'Mutating writes occur through mediated broker path: write roots constrained by capability scope, direct writes outside approved roots denied, write intent linked to transaction and receipt chain.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 3.3 Filesystem mutation path
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 3.3 Filesystem mutation path

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Filesystem mutation path

Mutating writes occur through mediated broker path:
- write roots constrained by capability scope,
- direct writes outside approved roots denied,
- write intent linked to transaction and receipt chain.
