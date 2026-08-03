---
id: GONI-SPEC-54F12A29F323
title: 8. Receipt Contract
type: specification
status: draft
implementation_state: specified_only
proposition: Delegated financial actions MUST emit receipts sufficient to reconstruct why the agent was authorized, what it negotiated, why it selected or rejected an offer, and what was settled.
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
  heading: 8. Receipt Contract
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 8. Receipt Contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Receipt Contract

Delegated financial actions MUST emit receipts sufficient to reconstruct why
the agent was authorized, what it negotiated, why it selected or rejected an
offer, and what was settled.

Receipt metadata SHOULD include:

- treasury ref,
- mandate hash or mandate ref,
- Work Order ref,
- policy hash,
- budget reservation and release refs,
- spend cap and final amount,
- reservation price and target price where disclosure policy permits,
- counterparty ref and counterparty risk basis,
- permitted instrument and settlement instrument,
- price evidence,
- quality evidence,
- offer and counteroffer summaries,
- approval decision and approver ref when applicable,
- escalation or omission reasons,
- settlement ref,
- compensation or dispute path,
- revocation status,
- receipt chain refs.

Receipts MUST NOT store raw private contract text, credentials, payment secrets,
or unnecessary personal data by default. They store refs, hashes, bounded
summaries, and replay metadata compatible with `REC-01`.

Valid escalation and omission reasons include `above_approval_threshold`,
`above_reservation_price`, `counterparty_denied`, `quality_below_threshold`,
`risk_too_high`, `terms_non_standard`, `instrument_denied`, `mandate_expired`,
`mandate_revoked`, `insufficient_evidence`, and `policy_denied`.
