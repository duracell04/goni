---
id: GONI-PROPOSAL-METIOR-GTL-20261031
title: Mêtior and Goni Token Ledger — monetary numéraire and settlement integration
type: proposal
status: draft
implementation_state: specified_only
proposition: Explore Mêtior (MEΩ) as an independently governed monetary reference unit for pricing and accounting within Goni Token Ledger, with settlement assets and legal claims explicitly separated.
domains:
  - system
  - software
aliases:
  - MEΩ-GTL integration
  - monetary numéraire for Goni
relations:
  - type: related_to
    target: GONI-PROPOSAL-GTL-20261031
sources: []
artifacts: []
uncertainty: Research hypothesis only. Benchmark quality, oracle, tokenization, settlement, reserves, legal characterization, economic utility, and protocol interfaces remain undecided.
legacy: []
---

# Mêtior (MEΩ) × Goni Token Ledger: exploratory integration

**Recorded:** 2026-10-31  
**Status:** Brainstorming; no implementation, token issuance, or architectural adoption authorized.

## Central concept

Mêtior defines a *numéraire* (monetary measurement unit), Goni Token Ledger (GTL) establishes asset identities and settlement ownership, and the sovereign Goni kernel governs a user's authority to initiate transfers. The components should remain independent so Goni can use MEΩ as one selectable pricing/benchmark unit without requiring a native cryptocurrency or a new blockchain.

- **MEΩ reference layer:** publishes versioned index values, component definitions, weights, methodology, observation time, and data provenance.
- **GTL ownership/settlement layer:** validates asset-specific mint, spend, split, merge, freeze, and recovery rules; serial-identified UTXO lots remain one candidate model.
- **Goni local authority layer:** user mandates, keys, capability checks, consent, egress policies, and transaction receipts.
- **Optional cooperative compute layer:** quotes compute work in MEΩ, escrows a supported settlement asset, and pays on contractually accepted evidence.

## Mêtior as specified in its source repository

Source: https://github.com/duracell04/Metior/blob/main/README.md (read 2026-10-31). MEΩ is proposed as a capitalization-weighted monetary benchmark built from fiat M2, gold/silver stocks, and selected crypto free float. For component j:

MC_j^USD(t) = P_j^USD(t) × Q_j(t)
M_world(t) = Σ_j MC_j^USD(t)
w_j(t) = MC_j^USD(t) / M_world(t)
P_MEΩ^USD(t) = κ × M_world(t), with example κ = 10^-6.

The repository describes daily rule-based recalculation, component exclusion for stale data, open publication of weights, optional tokenization, and a proposed GINα performance measure. Its current code uses a static 2025-10-08 snapshot, so a live independently validated index/oracle is a future deliverable. MEΩ is defined as a measurement standard, not a custodial reserve claim or existing stablecoin.

## Distinct financial instruments

1. **Index/numéraire:** a computed value with no holder, token supply, or redemption rights.
2. **MEΩ-denominated obligation:** e.g. a compute invoice priced at 0.000001 MEΩ and payable in a selected settlement asset using an agreed observation and conversion rule.
3. **MEΩ-referenced token:** an issued instrument whose contractual payoff references MEΩ; its price may deviate from the reference without credible issuance, redemption, hedging, or backing.
4. **Basket-backed redeemable claim:** a separately governed legal and custodial instrument with enforceable redemption and reserve verification.

A fixed token supply and a published index formula alone cannot enforce a token's market price. Cryptographic proofs of data integrity do not constitute proof of reserve ownership or redemption solvency.

## Candidate transaction contract

Quote = (job_or_obligation_id, amount_MEΩ, index_methodology_version, observation_time_or_window, settlement_asset, conversion_source, expiry, fee_rule, rounding_rule, acceptance_condition).

At settlement:
1. Resolve a valid signed/versioned MEΩ observation.
2. Compute the contractual settlement amount in the agreed asset with defined precision and fallback rules.
3. Have the Goni kernel authorize spending under current policy.
4. Execute the GTL transfer, await the specified finality threshold, and issue linked pricing, authorization, computation (if any), and payment receipts.

Illustrative accounting only: if a service costs 0.000001 MEΩ and 1 MEΩ is quoted at USD 100,000,000, its reference price is USD 100. The actual asset paid, its FX price, and conversion timestamp must be separately agreed. These are example numbers, not live observations.

## Research questions and risk audit

- **Benchmark validity:** Does a sum of fiat M2 converted to USD, above-ground precious metal stocks, and crypto free-float market capitalization measure a coherent economic quantity? Test overlap, double counting, monetary aggregates across jurisdictions, valuation sensitivity, revision history, and constituent coverage.
- **Purchasing power:** MEΩ-relative returns are benchmark-relative, not automatically inflation-free, purchasing-power-preserving, or 'pure alpha'. Validate against CPI, real effective exchange rates, SDR, gold, and diversified benchmark alternatives across historical crises.
- **Oracle integrity:** Pin methodology version, input source, timestamp, signed observation, freshness bounds, revision policy, availability, and disputed-data fallback. Evaluate manipulation and correlated-source risks.
- **Supply and economics:** Evaluate whether tokenization is useful at all; if so distinguish index tracker, service credit, floating asset, and redeemable claim. Test market making, liquidity, collateral, and redemption feasibility.
- **Serial fungibility:** A GTL payment asset with public serial histories may face taint-based price discrimination even when its nominal unit is homogeneous.
- **Legal treatment:** Evaluate index licensing, issuance and redemption claims, custody, AML, payments, securities/derivatives classification, and jurisdictional enforceability before any issuance.
- **Governance:** Keep MEΩ methodology governance separate from GTL ownership/recovery governance and from Goni kernel user authority; specify upgrade/version handling and conflict resolution.

## Minimal experimental path

1. Use Mêtior's published static snapshot as a clearly labeled synthetic/offline test input; check its internal arithmetic and provenance.
2. Specify a versioned, signed MEΩ price-observation interface.
3. Build a deterministic mock GTL transaction priced in MEΩ but settled in a synthetic existing asset; prove conservation, authorization, conversion rounding, and receipt linkage.
4. Compare MEΩ-denominated prices and historical portfolio returns against USD, CHF, CPI-adjusted units, SDR, and gold using independently sourced data.
5. Only then decide whether to adopt an MEΩ pricing adapter, whether GTL should settle real assets, and whether any tokenized MEΩ instrument has economic or legal merit.

**Boundary:** This proposal records a possible interoperability design. It does not merge the Metior and Goni projects, adopt MEΩ as mandatory currency, or assert that MEΩ is backed, stable, or empirically superior.
