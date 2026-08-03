---
id: GONI-SPEC-CEF61F69C96B
title: 3.1 DelegatedAgentTreasury
type: specification
status: draft
implementation_state: specified_only
proposition: DelegatedAgentTreasury is the logical control-plane account through which a principal grants an agent bounded authority to search, negotiate, commit, and pay under predefined constraints.
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
  heading: 3.1 DelegatedAgentTreasury
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 3.1 DelegatedAgentTreasury

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 DelegatedAgentTreasury

`DelegatedAgentTreasury` is the logical control-plane account through which a
principal grants an agent bounded authority to search, negotiate, commit, and
pay under predefined constraints.

A treasury is not necessarily a wallet. It may be backed by a payment
instrument, smart account, credential provider, enterprise procurement system,
escrow service, or manual approval path. Regardless of backing, DAT-01 treats
the treasury as kernel-governed authority and budget state.

Minimum logical fields:

```yaml
treasury_ref:
principal_ref:
agent_ref:
policy_hash:
permitted_instruments:
spend_caps:
counterparty_policy_ref:
approval_policy_ref:
revocation_ref:
valid_from:
expires_at:
receipt_ref:
provenance:
```
