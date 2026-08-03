---
id: GONI-SPEC-671622AA3D69
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Duplicate mutating requests with same idempotency tuple do not create duplicate external effects.
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md
  heading: Conformance tests
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- Duplicate mutating requests with same idempotency tuple do not create
  duplicate external effects.
- Rollback leaves no partial local state mutation.
- Irreversible external effects emit compensation-required status when rollback
  is impossible.
- Every terminal transaction state has a verifiable receipt.
