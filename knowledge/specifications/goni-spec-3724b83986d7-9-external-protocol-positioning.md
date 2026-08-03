---
id: GONI-SPEC-3724B83986D7
title: 9. External Protocol Positioning
type: specification
status: draft
implementation_state: specified_only
proposition: External commerce and payment protocols are adapters to the DAT-01 authority model, not replacements for it.
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
  heading: 9. External Protocol Positioning
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 9. External Protocol Positioning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. External Protocol Positioning

External commerce and payment protocols are adapters to the DAT-01 authority
model, not replacements for it.

- AP2 is relevant because signed mandate chains, open mandates, closed
  mandates, and autonomous mandate verification map to delegated purchase
  authority and evidence.
- x402 is relevant because HTTP-native `402 Payment Required` flows can support
  machine clients paying for APIs, data, or digital content under a mandate.
- Ethereum account abstraction and ERC-6900-style modular smart accounts are
  relevant because programmable validation, session keys, spend limits,
  subscriptions, and role-based authority can enforce parts of a treasury or
  negotiation mandate.

Goni MUST treat these as optional rail-specific implementations. A protocol
adapter may satisfy mandate, settlement, or receipt requirements only when its
evidence is bound back to the Work Order, policy decision, and Goni receipt
chain.
