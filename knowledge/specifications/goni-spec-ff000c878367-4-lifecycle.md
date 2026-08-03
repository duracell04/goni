---
id: GONI-SPEC-FF000C878367
title: 4. Lifecycle
type: specification
status: draft
implementation_state: specified_only
proposition: 'Delegated financial execution MUST follow this logical lifecycle: Compile a Work Order and Done Contract through DELEG-INT-01.'
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
  heading: 4. Lifecycle
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 4. Lifecycle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Lifecycle

Delegated financial execution MUST follow this logical lifecycle:

1. Compile a Work Order and Done Contract through `DELEG-INT-01`.
2. Bind the Work Order to a `DelegatedAgentTreasury` and
   `NegotiationMandate`.
3. Validate mandate authority, counterparty scope, instrument scope, time
   window, and policy hash.
4. Reserve the relevant budget through the transactional tool path.
5. Search, compare, request quotes, negotiate, or evaluate offers under the
   mandate.
6. Compute risk-adjusted value and collect required evidence.
7. Commit only if the offer satisfies mandate, policy, and approval rules.
8. Escalate when approval thresholds, ambiguity, risk, or non-standard terms
   exceed the mandate.
9. Settle payment through a mediated transaction when authorized.
10. Emit receipts for mandate use, budget reservation, negotiation basis,
    approval or escalation, settlement, and revocation status.

This lifecycle is a control-plane requirement. It does not require every vendor
or protocol to expose explicit negotiation APIs.
