---
id: GONI-PRINCIPLE-9249B8613105
title: 7. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: No direct socket egress from tool runner context.
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
  heading: 7. Invariants
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 7. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Invariants

- No direct socket egress from tool runner context.
- No filesystem mutation without capability-scoped mediated path.
- No side effect without policy decision artifact.
- No mediated side effect without receipt emission.
