---
id: GTL-01
title: "Goni Token Ledger: individually identified monetary units (exploratory)"
type: proposal
status: draft
implementation_state: specified_only
proposition: "Explore a Goni-compatible monetary settlement ledger with uniquely identified smallest units, conserved transaction lineage, fungible payment UX, and separately governed finality and optional recovery."
domains:
- agent
- network
- policy
- security
- system
aliases: []
relations:
- type: refines
  target: GONI-IMAP-ED288DB4EAF5
- type: depends_on
  target: GONI-DECISION-9AF49466170D
- type: depends_on
  target: GONI-SPEC-841BF8F599B9
sources: []
artifacts: []
uncertainty: "Hypothesis only; no issuance, ledger protocol, consensus, privacy, recovery authority, legal title, economic value or measured benefit has been selected or demonstrated."
legacy: []
---

# GTL-01 - Goni Token Ledger (brainstorm)

## Question and status

Could a Goni-compatible settlement system assign every smallest currency unit a persistent identity and verifiable lineage, while allowing ordinary fungible payments and optionally governed recovery of stolen property?

This is an **exploratory proposal**. It does not approve a cryptocurrency, issue real tokens, select a blockchain, establish legal ownership, or assert an implemented protocol. "NFT-like" refers to unique identity and provenance; literal ERC-721 tokens for each currency unit are only one, potentially inefficient, representation.

## Candidate design

Distinguish five objects:

1. **Economic asset:** named denomination and smallest integer quantum, with independent rules for issuance, value, reserves, fees, and redemption.
2. **Permanent unit identity:** a unique identifier such as (asset namespace, issuance ID, serial), tied to a minted smallest unit.
3. **Spendable lot:** one or more units represented by an unspent output, quantity, current control rule, policy version, and optional serial-range or commitment representation.
4. **Transaction lineage:** signed, ordered transitions between consumed and created outputs, retaining links to original issuance identities.
5. **Settlement state:** the accepted unspent ownership set and any restrictions, with an explicit consensus/finality mechanism.

The **unit identifier** persists across transfers; the **spendable-output identifier** changes. Current control under ledger rules must be distinguished from legally recognized property title.

An illustrative issuance of serials #00041-#00140 has 100 units. A payment of 35 units consumes the current output and creates a recipient output containing 35 units and a change output containing 65. No serial is duplicated or silently discarded. The selected smallest unit defines divisibility; range compression is an optimization, and fragmentation is a cost to measure.

For ordinary transfers, the intended serial-conservation invariant is:

    disjoint_union(serials(inputs)) = disjoint_union(serials(outputs))

Separate, explicitly authorized issuance and burn/redeem operations modify total supply. Any monetary fees must have an explicit accounting rule. Valid transfer requires genuine spending authority, unspent inputs, unit conservation, correct policy and a finalized network transition.

Candidate operations: MINT, TRANSFER, SPLIT, MERGE, FREEZE, RECOVER, BURN. SPLIT and MERGE could be standard multi-input/multi-output transactions rather than distinct opcodes. FREEZE and RECOVER apply only to asset classes whose governance grants those powers.

## Goni trust boundary

The Goni **kernel** retains mandate and capability validation, key-control policy, approvals, spending limits, network egress and receipts. The **shared ledger** validates asset uniqueness, transfer rules, double-spend prevention and finality under its independently specified consensus or validation model.

A model may propose a payment; the kernel must approve any effect under existing financial/transactional-tool contracts. Payment receipts reference mandate, authorization, settlement and dispute information without publishing payment credentials or private memory.

Offline nodes may inspect balances, prepare transactions or sign subject to local policy. Shared settlement finality normally requires network participation, and offline payments must carry an explicitly documented double-spend-risk assumption.

Private conversations, memory, full computation traces and personal data stay outside shared ledger consensus. Minimal commitments and identifiers can cross the boundary when justified by the payment contract.

The proposed monetary unit is distinct from existing Goni capability tokens, identity claims, execution receipts and work orders.

## Theft, dispute and recoverability (conditional)

Theft can be traced only to the extent the selected representation discloses or makes lineage provable; a unique identifier by itself does not establish who stole a key or who legally owns an asset.

A recoverable asset class could support an incident report, limited-duration freeze, independent review, appeal and narrowly scoped corrective reassignment. Such actions must leave append-only evidence and preserve original transfer history.

The protocol would have to specify who may trigger review, decision thresholds, quorum, separation of powers, false reports, cross-jurisdictional effect, innocent holders, compensation and the protection against malicious administrators. An ordinary signature from a stolen wallet key may be technically valid while the transfer is legally disputed. An allegation alone is insufficient grounds for irreversible reassignment.

An unrestricted bearer-like asset remains another coherent policy option. Which instrument is intended is an institutional decision, not a cryptographic default.

## Cooperative compute, if it becomes useful

A later network of sovereign Goni nodes might exchange services or verifiable compute jobs for payment. A kernel-authorized budget could be placed in escrow; peer execution and independently evaluated contract-specific evidence could determine payout or dispute. Keep computation evidence and financial settlement receipts distinct but linked.

Correctness is workload-dependent: exact computations, approximate results and stochastic AI inference need different evaluation rules. A signed execution receipt is not by itself a proof of correct or useful work. Payment settlement does not entail choosing proof-of-useful-work blockchain consensus.

## Competing designs, not predetermined selections

Compare ordinary fungible account/UTXO rails with separate provenance; public serial-identified UTXO lots; shielded commitments and zero-knowledge conservation; client-side validated ownership proofs; and conventional assets with legally defined freeze or dispute rules.

Potential technical comparisons for a separately sourced literature review include Bitcoin UTXOs and Ordinal Theory, ERC-3525, ERC-3643, RGB and private digital-cash protocols. Decide independently whether existing L1/L2 settlement, participant validation or new consensus is justified.

The value basis also remains open: native floating-price cryptocurrency, reserve-backed redeemable instrument, compute credit, or an existing external payment asset. Serial identity and NFT status alone do not create monetary backing or market value.

## Open decisions

- Must tracing be public, selectively disclosable, or private by default?
- Can economic fungibility coexist with serial-linked theft or sanctions labels?
- What are the rights of an innocent merchant who accepts an allegedly stolen unit?
- Which entity has intervention power, based on which legal authority and procedural safeguards?
- What are the real storage, proof, transaction fee, fragmentation, latency and privacy costs?
- How does the protocol enforce double-spend prevention and finality during offline operation, network partitions and reorganizations?
- Would Goni gain more from its own unit, or from mandate-governed settlement on established rails?

See GTL-OBJECTION-01 for the central critique and GTL-EVAL-01 for a proposed evaluation. Promotion requires measured technical, economic and legal evidence. Goni's core Delegation OS architecture continues to function without this ledger.
