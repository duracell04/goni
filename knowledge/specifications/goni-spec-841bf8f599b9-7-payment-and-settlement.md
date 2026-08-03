---
id: GONI-SPEC-841BF8F599B9
title: 7. Payment And Settlement
type: specification
status: draft
implementation_state: specified_only
proposition: Any payment, fund reservation, purchase, subscription, or financially binding commitment is a mutating external side effect and MUST use transactional tool semantics from SPEC-TXN-01.
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegated-agent-treasury.md
  heading: 7. Payment And Settlement
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 7. Payment And Settlement

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Payment And Settlement

Any payment, fund reservation, purchase, subscription, or financially binding
commitment is a mutating external side effect and MUST use transactional tool
semantics from `SPEC-TXN-01`.

Before settlement, the runtime MUST:

- validate capability and policy,
- reserve budget,
- verify idempotency support or create an idempotency key,
- confirm approval requirements,
- record counterparty and price evidence,
- classify compensation or dispute path.

Rollback is not assumed for external financial effects. Compensation,
cancellation, refund, dispute, or operator escalation paths MUST be recorded
when reversibility is incomplete.
