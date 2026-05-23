---
id: DAT-01
type: SPEC
status: specified_only
---
# DAT-01 - Delegated Agent Treasury
DOC-ID: DAT-01
Status: Specified only / roadmap

This spec defines the control-plane contract for agents that search, bargain,
contract, and pay under delegated financial authority. A commercial agent is
not treated as an entity that simply "has money"; it is a delegated economic
actor operating inside a bounded mandate from a principal.

DAT-01 is specified only. It does not add a shipping schema table, change the
`/v1/chat/completions` API, require a wallet implementation, or require any
specific payment rail. Implementations may later map this contract onto cards,
bank rails, stablecoins, smart accounts, signed mandate protocols, or HTTP
payment protocols, but the Goni control-plane authority model is independent of
those adapters.

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

## 2. Economic Model

An agent acting under DAT-01 optimizes expected net value, not nominal lowest
price:

```text
ExpectedNetValue = Utility - Price - Risk - MonitoringCost
```

Where:

- `Utility` is the principal's expected value from the good, service, quality,
  timing, or contract terms.
- `Price` is the monetary consideration paid or committed.
- `Risk` includes counterparty, fraud, delivery, compliance, legal, privacy,
  data leakage, settlement, and quality uncertainty.
- `MonitoringCost` includes approvals, audits, evidence collection,
  reconciliation, and transaction overhead.

The agent's goal is the best risk-adjusted price within the authorized mandate.
A lower nominal price MUST NOT be selected when it violates counterparty,
quality, legal, privacy, delivery, or approval constraints.

## 3. Normative Objects

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

### 3.3 Financial Decision Rights

A mandate MUST define the agent's economic decision rights, including:

| Variable | Function |
| --- | --- |
| `spend_cap` | Limits downside financial exposure. |
| `reservation_price` | Defines maximum willingness to pay. |
| `target_price` | Guides bargaining and comparison. |
| `approved_counterparties` | Reduces adverse selection and blocked vendors. |
| `quality_thresholds` | Prevents false savings from lower-quality offers. |
| `time_window` | Limits mandate duration. |
| `approval_thresholds` | Controls high-risk discretion. |
| `revocation_ref` | Preserves principal authority. |
| `evidence_requirements` | Defines the proof needed before commitment. |
| `negotiation_rules` | Structures bargaining behavior. |

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

## 7. Payment And Settlement

Any payment, fund reservation, purchase, subscription, or financially binding
commitment is a mutating external side effect and MUST use transactional tool
semantics from `SPEC-TXN-01`.

Before settlement, the runtime MUST:

- validate capability and policy,
- reserve budget,
- verify idempotency support or create an idempotency key,
- confirm approval requirements,
- record counterparty and price evidence,
- classify compensation or dispute path.

Rollback is not assumed for external financial effects. Compensation,
cancellation, refund, dispute, or operator escalation paths MUST be recorded
when reversibility is incomplete.

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

## 11. Related Specs

- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Policy Language](/blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md)
- [Transactional Tool Execution](/blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Execution metering and budget units](/blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md)

## 12. External References

- [AP2 specification](https://ap2-protocol.org/ap2/specification/)
- [AP2 agent authorization](https://ap2-protocol.org/ap2/agent_authorization/)
- [Coinbase x402 docs](https://docs.cdp.coinbase.com/x402/welcome)
- [EIP-4337](https://eips.ethereum.org/EIPS/eip-4337)
- [ERC-6900](https://erc6900.io/)

## Conformance Tests

- No financial negotiation or payment can occur without a Work Order, policy
  hash, mandate, and budget reservation.
- A cheaper counterparty is rejected or escalated when risk, quality, legal,
  privacy, delivery, or counterparty constraints violate the mandate.
- Payment above the approval threshold escalates before settlement.
- Expired or revoked mandates deny further negotiation and payment.
- Every committed financial side effect emits a receipt with mandate, policy,
  counterparty, price, approval, and settlement refs.
- A failed or partially completed settlement records retry, compensation,
  dispute, or operator-escalation status.
- Subdelegated financial work cannot exceed the original mandate scope.

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
