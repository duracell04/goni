---
id: GONI-SPEC-8C565AF51393
title: Acceptance Fixtures
type: specification
status: draft
implementation_state: specified_only
proposition: 'Approved vendor discount negotiation: the agent negotiates with an approved vendor under target price, satisfies quality thresholds, settles within cap, and emits a complete receipt chain.'
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
  heading: Acceptance Fixtures
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# Acceptance Fixtures

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Acceptance Fixtures

- Approved vendor discount negotiation: the agent negotiates with an approved
  vendor under target price, satisfies quality thresholds, settles within cap,
  and emits a complete receipt chain.
- Unapproved vendor low-price rejection: a cheaper offer from a denied
  counterparty is rejected or escalated with `counterparty_denied`.
- AP2-style signed mandate purchase: an adapter provides open and closed
  mandate evidence linked to the Work Order and Goni mandate receipt.
- x402 pay-per-use API request: an agent pays for an API response only when the
  requested resource, amount, instrument, and receipt metadata fit the mandate.
- Smart-account spend-limit denial: a payment above the account or mandate
  spend limit is denied and receipt-linked to the policy decision.
