---
id: GONI-SPEC-3D7299419769
title: 6. Negotiation And Price Discovery
type: specification
status: draft
implementation_state: specified_only
proposition: The agent MAY use comparison shopping, RFQs, reverse auctions, bilateral negotiation, posted-price acceptance, marketplace search, or protocol-mediated purchase flows when the mandate permits them.
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
  heading: 6. Negotiation And Price Discovery
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 6. Negotiation And Price Discovery

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Negotiation And Price Discovery

The agent MAY use comparison shopping, RFQs, reverse auctions, bilateral
negotiation, posted-price acceptance, marketplace search, or protocol-mediated
purchase flows when the mandate permits them.

The agent MUST evaluate offers against:

- price and total cost,
- quality and service thresholds,
- counterparty authorization,
- delivery or fulfillment confidence,
- legal and contractual terms,
- privacy and data exposure,
- payment instrument constraints,
- approval thresholds,
- required evidence.

An agent MUST NOT optimize price in isolation. If an offer is cheaper but
violates mandate constraints or materially increases risk, the agent MUST reject
or escalate it with a bounded reason.
