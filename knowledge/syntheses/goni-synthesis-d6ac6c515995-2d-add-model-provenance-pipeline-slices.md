---
id: GONI-SYNTHESIS-D6AC6C515995
title: 2d) Add model provenance pipeline slices
type: synthesis
status: draft
implementation_state: specified_only
proposition: Add a model install fixture that produces a ModelManifest and InstallReceipt.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 2d) Add model provenance pipeline slices
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 2d) Add model provenance pipeline slices

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2d) Add model provenance pipeline slices
- Add a model install fixture that produces a ModelManifest and InstallReceipt.
- Add a model eval fixture that produces an EvalReceipt.
- Add a rollback fixture that proves RollbackRef can restore the prior approved
  bundle or quarantine a failed candidate.
