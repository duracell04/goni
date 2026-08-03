---
id: GONI-SPEC-AEAEFE1AD536
title: Conformance Tests
type: specification
status: draft
implementation_state: specified_only
proposition: No financial negotiation or payment can occur without a Work Order, policy hash, mandate, and budget reservation.
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
  heading: Conformance Tests
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# Conformance Tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance Tests

- No financial negotiation or payment can occur without a Work Order, policy
  hash, mandate, and budget reservation.
- A cheaper counterparty is rejected or escalated when risk, quality, legal,
  privacy, delivery, or counterparty constraints violate the mandate.
- Payment above the approval threshold escalates before settlement.
- Expired or revoked mandates deny further negotiation and payment.
- Every committed financial side effect emits a receipt with mandate, policy,
  counterparty, price, approval, and settlement refs.
- A failed or partially completed settlement records retry, compensation,
  dispute, or operator-escalation status.
- Subdelegated financial work cannot exceed the original mandate scope.
