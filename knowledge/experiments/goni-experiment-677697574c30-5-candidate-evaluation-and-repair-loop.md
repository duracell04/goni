---
id: GONI-EXPERIMENT-677697574C30
title: 5. Candidate evaluation and repair loop
type: experiment
status: draft
implementation_state: not_applicable
proposition: 'ITCR is a propose -> check -> revise loop: Propose: produce one or more candidate plans.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 5. Candidate evaluation and repair loop
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 5. Candidate evaluation and repair loop

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Candidate evaluation and repair loop

ITCR is a propose -> check -> revise loop:

1) Propose: produce one or more candidate plans.
2) Check: score candidates using auxiliary criteria beyond next-token likelihood
   (validators, rule checks, verifier models, constraint evaluation).
3) Revise: repair or resample when checks fail.

The loop exits early if a candidate satisfies all checks or if budgets expire.
