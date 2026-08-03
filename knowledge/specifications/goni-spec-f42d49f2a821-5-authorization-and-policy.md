---
id: GONI-SPEC-F42D49F2A821
title: 5. Authorization And Policy
type: specification
status: draft
implementation_state: specified_only
proposition: Financial delegation is default-deny.
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
  heading: 5. Authorization And Policy
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 5. Authorization And Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Authorization And Policy

Financial delegation is default-deny. No agent may negotiate, commit, reserve
funds, or pay unless all of the following exist:

- Work Order reference,
- policy hash,
- valid treasury reference,
- valid negotiation mandate,
- budget reservation,
- permitted payment or settlement instrument,
- receipt path.

The policy language remains the normative source for capability grants, spend
units, resource scopes, validity windows, and revocation. DAT-01 extends those
policy concepts to financial agency but does not replace `SPEC-POL-01`.

Mandates are non-transferrable unless an explicit delegation rule permits
subdelegation. If an agent delegates work to another agent, the downstream
agent's authority MUST be no broader than the original mandate and MUST remain
receipt-linked to the original principal grant.
