---
id: GONI-PRINCIPLE-8DBD117DF580
title: 6. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: The runtime rejects undeclared workflow hashes.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/visual-runtime.md
  heading: 6. Invariants
  revision: 4d6a56dfeb55430356f9e72b203b5df766df28e8
---

# 6. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Invariants

- The runtime rejects undeclared workflow hashes.
- The runtime rejects model bundle IDs that are not eligible for the requested
  visual task class.
- Outputs are content-addressed before memory update or export.
- Audit-grade visual runs preserve enough settings to support trace replay when
  the backend can run deterministically.
- Backend workflow logs are diagnostic only; canonical receipts are emitted by
  the Goni kernel.
