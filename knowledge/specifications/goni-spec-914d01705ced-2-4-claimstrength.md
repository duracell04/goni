---
id: GONI-SPEC-914D01705CED
title: 2.4 ClaimStrength
type: specification
status: draft
implementation_state: specified_only
proposition: 'ClaimStrength = proved | supported | unknown | disproved proved: complete-enough scope plus direct evidence.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.4 ClaimStrength
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.4 ClaimStrength

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.4 ClaimStrength

`ClaimStrength = proved | supported | unknown | disproved`

- `proved`: complete-enough scope plus direct evidence.
- `supported`: strong evidence, but not exhaustive.
- `unknown`: scope incomplete or decisive evidence missing.
- `disproved`: complete-enough scope plus contrary evidence.
