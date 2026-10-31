---
id: GONI-PROPOSAL-GTL-20261031
title: Goni Token Ledger — serial-identified monetary units and governed settlement
type: proposal
status: draft
implementation_state: specified_only
proposition: Explore a Goni-compatible ledger that combines uniquely identifiable monetary units, economically fungible balances, verifiable ownership lineage, and optional policy-governed recovery.
domains:
  - system
  - security
  - software
aliases:
  - GTL
  - serial-numbered digital cash
relations: []
sources: []
artifacts: []
uncertainty: Brainstorming only. Monetary design, consensus, privacy, finality, recovery authority, legal treatment, and implementation remain undecided.
legacy: []
---

# Goni Token Ledger (GTL): research proposal

**Recorded:** 2026-10-31  
**Maturity:** Exploratory idea, not an approved protocol, currency launch, or implementation commitment.

## Motivation and thesis

Explore whether a monetary system can combine the convenience of fungible money with NFT-like *individual identity*: each base monetary unit has a unique serial and verifiable lineage, while wallets show ordinary aggregated balances. Distinguish (1) identifying a stolen unit, (2) restricting its movement, and (3) legally recovering ownership. These are separate capabilities requiring different trust and governance assumptions.

This idea is adjacent to Goni's local-first sovereign delegation, deterministic authority, capability-mediated effects, and cryptographic receipts. It is not a replacement for the Goni kernel or a requirement that Goni have a native cryptocurrency.

## Proposed representation and transitions

Candidate: serial-numbered UTXO lots, with compressed ranges where possible. Distinguish persistent monetary-unit identity from the current unspent transaction-output identifier. An output records asset class, serial membership or commitment, quantity, spending condition, parent reference, policy version, and status. A wallet presents aggregated monetary value.

Illustrative transfer: Alice owns serials 00041–00140 (100 units); sending 35 consumes the original output and creates Bob's 00041–00075 and Alice's change 00076–00140. New output identifiers are created; original serial identity and lineage persist.

For an ordinary transfer, the multiset of input serials must equal the multiset of output serials, and every serial must appear exactly once in spendable state. Mint and burn are distinct authorized operations. Fee handling, denominations, fractional units, splits, merges, fragmented serial sets, and commitment-proof efficiency require specification.

Candidate transition vocabulary:
- MINT: authorized issuance of new unit identities.
- TRANSFER: spend valid inputs and assign new outputs.
- SPLIT / MERGE: reorganize lots while preserving identity and quantity.
- FREEZE: restrict spending subject to the asset's governance contract.
- RECOVER: append an authorized corrective ownership transition.
- BURN: retire units, potentially as part of redemption.

Validation must cover authorization, unspent inputs, conservation, issuance bounds, policy version, replay/double-spend prevention, and consensus finality. A cryptographic commitment alone does not establish canonical ordering or finality.

## Architectural boundaries

- **Local Goni kernel:** user intent, keys, mandates, spending approval, capability checks, local receipts, and privacy controls.
- **Shared settlement layer:** validates monetary state transitions and establishes canonical ownership/finality under a defined consensus or settlement mechanism.
- **Goni cooperative compute:** optionally submits execution receipts and obtains settlement under separately specified verification and acceptance contracts.
- **Private cognition/memory:** remains locally governed; a public ledger carries only necessary settlement information or cryptographic commitments.

Keep distinct the semantics of monetary assets, compute receipts, capability tokens, and ownership/right-bearing tokens. A compute receipt evidences claimed work under its verification boundary; it does not alone prove that stochastic AI output is useful or correct.

## Theft, disputes, and legal rights

Permanent identifiers facilitate tracing, but a compromised valid signature is not intrinsically distinguishable from an authorized transfer. Recovery needs a governance contract, evidentiary standard, dispute and appeal process, finality policy, and protection for innocent recipients. A corrective transition should preserve historical evidence rather than rewrite prior transactions.

Open question: whether asset classes are irreversibly bearer-controlled, recoverable through limited intervention, or issued as regulated claims. The governing legal regime must define the effect of ledger registration and transfers; ledger entries alone do not establish legal title.

## Privacy, scaling, and alternative designs

Compare:
1. Public serial/UTXO ledger: easy public tracing; serious privacy and taint/fungibility concerns.
2. Shielded serial ledger: private spending proofs and selective disclosure; harder recovery and verification design.
3. Client-side-validated ownership histories with shared anti-double-spend anchors: more local privacy, but history delivery, availability, and synchronization challenges.

Evaluate range compression, fragmentation, verification cost, data availability, audit proofs, transaction throughput, network fees, and reorganization/finality behavior. An existing blockchain or rollup may be preferable to building a new consensus network. Investigate Bitcoin Ordinals, ERC-3525, ERC-3643, RGB/client-side validation, and zero-knowledge systems as relevant precedents, without assuming equivalence or originality.

## Economics and network role

Possible economic models: (a) freely floating native token; (b) reserve-backed stable-value claim; (c) service-denominated compute credit. Unique identity does not itself produce backing, price stability, demand, or liquidity. Analyze monetary policy, issuance, redemption, legal classification, and regulatory compliance separately.

Potential compute-settlement flow: a requester escrows funds; a provider performs a job; independently specified verification and acceptance determine release or dispute; the settlement receipt links payment to execution evidence. Proof-of-useful-work as a consensus mechanism is a separate research hypothesis.

## Candidate formal model

GTL = (U, O, T, V, G), where U is identified monetary units; O current ownership/spend state; T valid transitions; V validation, ordering, and finality; G issuance, intervention, and dispute governance.

Core invariants to test:
1. A unit is represented by at most one currently spendable output.
2. Ordinary transfers conserve serial membership and monetary quantity, with explicit fees.
3. Only authorized mint/burn/recovery operations alter supply or exceptional ownership.
4. Signatures, spending conditions, and policy are validated against canonical state.
5. History is append-only and independently reconstructable under specified data availability.
6. Local Goni functionality remains available offline; shared settlement requires network finality.

## Unresolved decisions and next experiments

- Is the primary goal traceability, recoverability, payment privacy, compute settlement, or tokenized legal rights?
- What constitutes ownership: possession of a key, a contractual claim, or a legally registered right?
- Which actors can freeze/recover, under what safeguards, and with what consequences for innocent purchasers?
- What privacy disclosures are compulsory, optional, or technically impossible?
- Which existing settlement substrate provides adequate finality, availability, cost, and interoperability?
- Can compressed serial ranges outperform conventional fungible balances under realistic fragmentation?
- Is a native GONI monetary asset economically necessary?

**Next step:** write a minimal deterministic reference state machine and property-based tests using synthetic wallets, serials, and transfers. Benchmark against a conventional fungible UTXO/balance model before selecting consensus or designing issuance. Preserve this proposal as research until separate governance decisions promote individual claims.
