---
id: GONI-SPEC-907D94397E5F
title: 1. Purpose
type: specification
status: draft
implementation_state: specified_only
proposition: Agent commerce is a delegation, contracting, and price-discovery problem before it is a payment problem.
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
  heading: 1. Purpose
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose

Agent commerce is a delegation, contracting, and price-discovery problem before
it is a payment problem. The principal owns the preference, budget, liability,
and authority. The agent executes search, comparison, negotiation, contracting,
and settlement only within the mandate granted by the principal and enforced by
kernel policy.

DAT-01 defines:

- `DelegatedAgentTreasury`, the control-plane account through which a principal
  grants bounded financial authority to an agent,
- `NegotiationMandate`, the bargaining-control object that constrains price,
  counterparties, quality, risk, approvals, evidence, and revocation,
- the expected net value objective for financial delegation,
- the lifecycle from Work Order to mandate, negotiation, payment, and receipt,
- the receipt and conformance requirements for delegated financial actions.
