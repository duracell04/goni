# Toward an Agent-Native Internet
**AI agents, transaction-cost economics, programmable commerce, and digital trust**

*Research note · 9 November 2026 · Proposed architecture, not implementation*

> **Status and authority.** This note is exploratory and non-canonical. It does not change GONI's knowledge graph, promote a draft, or assert that the described execution, payment, and enforcement capabilities have been implemented or independently validated.

## Thesis

The internet may evolve from human-operated navigation of websites toward **intent-based, machine-executable economic coordination**. Authorized agents could discover information, evaluate alternatives, negotiate, reserve services, and complete transactions through machine-readable interfaces, other agents, or browser automation.

The fundamental opportunity is not a faster browser but a **governed execution infrastructure** that connects information access, delegated authority, payments, contractual commitments, and independently verifiable outcomes.

## Economic mechanism

Coase (1937) established the significance of transaction costs in economic organization. Malone, Yates, and Benjamin (1987) argued that information technologies can change the cost of coordinating markets and hierarchies. Autonomous agents could extend this mechanism by substituting software for some human search and transaction work.

The relevant criterion is **total system cost**, not merely browsing speed or model-token consumption:

```text
Total transaction cost =
    search + comparison + coordination + contracting + settlement
  + computation + permission management + verification + monitoring
  + expected fraud/error losses + exceptions + dispute resolution
```

An agent-mediated workflow creates economic value only when it achieves a comparably reliable and legally satisfactory outcome at lower *expected* total cost than the relevant alternative. Automation may relocate rather than eliminate transaction costs.

## Architecture: different interfaces, one authority boundary

| Layer | Function | Examples / status |
| --- | --- | --- |
| Discovery | Find information and counterparties | Search engines, browser agents, structured catalogues |
| Interaction | Access services and coordinate actions | Browser/DOM, APIs, MCP, A2A |
| Commerce | Express offers, orders and reservations | UCP, ACP; emerging specifications |
| Authorization | Bind actions to the principal's mandate | Scoped credentials, spending ceilings, approvals, expiration |
| Settlement | Transfer value under permitted conditions | Banks, cards, AP2, x402, wallets, escrow |
| Verification and remedy | Confirm outcomes, correct failures | Provider receipts, reconciliation, cancellation, refunds, disputes |

The browser remains a **compatibility surface** for websites designed for humans; a machine-readable API or agent protocol can be preferable when it enables reliable, lower-cost operations. A protocol message, model response, clicked button, or signed transaction is not proof that a reservation, service delivery, or economic obligation was completed.

The proposed execution sequence is:

```text
Human intent and policy
  -> evidence-gathering agent
  -> proposed action
  -> deterministic authorization
  -> API / browser / remote agent
  -> provider-side effect
  -> independent confirmation, reconciliation, and recovery
```

This aligns with GONI's existing design separation between **model reasoning and operational authority**. Relevant canonical starting points include [DelegatedAgentTreasury](../knowledge/specifications/goni-spec-cef61f69c96b-3-1-delegatedagenttreasury.md), [kernel-mediated effect classes](../knowledge/specifications/goni-spec-d268994a7cbe-4-effect-classes.md), and [separating intelligence from authority](../knowledge/theses/goni-thesis-7bfb74017d50-2-core-thesis-separating-intelligence-from-authority.md). Those are specified-only contracts, not evidence of a working universal agent-commerce runtime.

## Programmable commitments and reversibility

Blockchain settlement and smart contracts can offer shared, programmable transaction conditions but do **not** make ordinary finalized transactions intrinsically reversible. Three mechanisms must be distinguished:

1. **Conditional settlement:** escrow, holds, staged release, and cancellation windows prevent funds from being released until a defined condition is satisfied.
2. **Protocol-specific reversible tokens:** Wang, Wang, and Boneh (2022) propose ERC-20R/ERC-721R designs with a limited dispute period and a judgment mechanism. These are specialized token designs, not generic blockchain rollback.
3. **Compensating remedies:** refunds, chargebacks where available, or authorized corrective transfers address a completed transaction without rewriting its historical record.

Conventional financial rails can be preferable when they already provide identity, consumer protection, fraud management, and dispute handling at lower overall cost. Instrument choice and legal enforceability are separate design questions.

## Empirical state and research questions

Mind2Web (NeurIPS 2023) and WebArena (ICLR 2024) establish research environments for web-agent interaction; neither proves unrestricted reliable autonomous commerce. A2A, UCP, ACP, AP2, and x402 cover **different partial interoperability or payment functions**, not a complete universal trust layer.

McKinsey (2025) forecasts, under scenarios, that AI agents *could mediate* approximately **US$3–5 trillion of global consumer commerce by 2030**; this is a forecast rather than realized volume. Visa, BILL, and Billtrust announced a live B2B payment pilot on **20 October 2026** in which agents execute **already approved** invoice payments, rather than independently deciding whether to pay.

The key research questions are: Can the architecture reduce total transaction costs at equal reliability? Can an agent's authority be checked independently of its reasoning? How are completed outcomes verified across counterparties? Which recovery and dispute mechanisms apply to each instrument and jurisdiction? How should responsibility be allocated among principals, agents, merchants, and payment providers?

## Selected references

### Academic literature

1. Coase, R. H. (1937). [The Nature of the Firm](https://doi.org/10.1111/j.1468-0335.1937.tb00002.x). *Economica*, 4(16), 386–405.
2. Malone, T. W., Yates, J., & Benjamin, R. I. (1987). [Electronic Markets and Electronic Hierarchies](https://doi.org/10.1145/214762.214766). *Communications of the ACM*, 30(6), 484–497.
3. Deng, X., et al. (2023). [Mind2Web: Towards a Generalist Agent for the Web](https://osu-nlp-group.github.io/Mind2Web/). *NeurIPS*.
4. Zhou, S., et al. (2024). [WebArena: A Realistic Web Environment for Building Autonomous Agents](https://webarena.dev/). *ICLR*.
5. Wang, K., Wang, Q., & Boneh, D. (2022). [ERC-20R and ERC-721R: Reversible Transactions on Ethereum](https://arxiv.org/abs/2208.00543). *arXiv preprint*.

### Primary technical specifications

6. [Agent2Agent (A2A) Protocol](https://github.com/a2aproject/A2A), Linux Foundation project.
7. [Universal Commerce Protocol (UCP)](https://developers.googleblog.com/under-the-hood-universal-commerce-protocol-ucp/), Google (2026).
8. [Agentic Commerce Protocol (ACP)](https://github.com/agentic-commerce-protocol/agentic-commerce-protocol), OpenAI and Stripe (beta).
9. [Agent Payments Protocol (AP2)](https://cloud.google.com/blog/products/ai-machine-learning/announcing-agents-to-payments-ap2-protocol), Google (2025).
10. [x402](https://github.com/x402-foundation/x402), x402 Foundation.

### Industry observations and projections

11. McKinsey (17 October 2025). [The Agentic Commerce Opportunity](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-agentic-commerce-opportunity-how-ai-agents-are-ushering-in-a-new-era-for-consumers-and-merchants). *Scenario analysis*.
12. Visa (20 October 2026). [Visa, BILL and Billtrust Launch First Live Pilot for Trusted AI Agent Payments](https://corporate.visa.com/en/sites/visa-perspectives/press-releases/visa-bill-billtrust-launch-pilot.html). *Provider-reported pilot*.

**Research boundary:** These sources support an economic hypothesis, emerging protocol capabilities, and bounded empirical examples; they do not collectively demonstrate that a secure, general-purpose agent-native internet already exists.
