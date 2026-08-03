---
id: GONI-PRINCIPLE-FD9D88AD1223
title: 10. Safety Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: No financial negotiation or payment without a Work Order, policy hash, mandate, and budget reservation.
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
  heading: 10. Safety Invariants
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 10. Safety Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Safety Invariants

- No financial negotiation or payment without a Work Order, policy hash,
  mandate, and budget reservation.
- No nominal-price optimization that ignores risk, quality, legal, privacy, or
  approval constraints.
- No use of a revoked, expired, or superseded mandate.
- No payment or binding commitment above approval threshold without explicit
  approval.
- No subdelegation that expands the original principal grant.
- No payment credential, private key, card data, or secret may be stored in
  receipts or mandate summaries by default.
- No external settlement path may bypass transactional tool semantics.
- Every committed financial side effect MUST emit a receipt linked to the
  mandate, policy decision, counterparty, price, approval state, and settlement
  outcome.
