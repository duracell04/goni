---
id: GONI-PRINCIPLE-8C0DB401EE76
title: 8. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: No mutating call without transaction context.
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
  heading: 8. Invariants
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 8. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Invariants

- No mutating call without transaction context.
- No mutating call without idempotency key.
- No external side effect without outcome classification.
- No transaction terminal state without receipt.
