---
id: GONI-SPEC-7650BCF6CE6D
title: 3.2 NegotiationMandate
type: specification
status: draft
implementation_state: specified_only
proposition: NegotiationMandate is the formal authorization that defines the agent's bargaining authority before payment or commitment occurs.
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
  heading: 3.2 NegotiationMandate
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 3.2 NegotiationMandate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 NegotiationMandate

`NegotiationMandate` is the formal authorization that defines the agent's
bargaining authority before payment or commitment occurs.

Minimum logical fields:

```yaml
mandate_ref:
principal_ref:
agent_ref:
work_order_id:
purpose:
spend_cap:
reservation_price:
target_price:
approved_counterparties:
permitted_instruments:
quality_thresholds:
negotiation_rules:
approval_thresholds:
time_window:
evidence_requirements:
revocation_ref:
policy_hash:
receipt_ref:
provenance:
```

`reservation_price` is the maximum price or worst acceptable exchange value the
agent may accept before escalating or declining. `target_price` guides
bargaining strategy but does not create authority to violate risk, quality,
time, or counterparty constraints. `approval_thresholds` define when the agent
must return to the principal or another authorized reviewer before commitment.
