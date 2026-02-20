# Kernel-Blockchain Security Mapping
Status: Specified only / roadmap

This document maps Goni's agentic-kernel primitives to blockchain-facing roles
using systems and security terminology.

This is an analytical framing document, not a normative protocol spec.

## 1. Scope and framing

A local agentic kernel can interact with blockchain systems in at least four
roles:
- hardened runtime host for a full node,
- reference monitor for signing and value-moving actions,
- provenance receipt emitter with external timestamp anchoring,
- validator safety guard for slashable consensus actions.

The core architectural stance remains:
- keys are not held by general agents,
- side effects are mediated through a small trusted core,
- authority is capability-scoped and revocable.

## 2. Role A: hardened runtime host for node software

When running a blockchain node, the kernel contribution is
compartmentalization plus resource governance:
- isolate node process and storage authority,
- constrain egress and RPC exposure,
- budget CPU, IO, and network to protect interactive QoS.

Isolation design points:
- container/sandbox boundary with explicit capabilities,
- microVM boundary where stronger isolation is required.

Relevant foundations:
- Firecracker microVM tradeoffs [[agache2020-firecracker]]
- capability confinement in UNIX contexts [[watson2010-capsicum]].

## 3. Role B: reference monitor for signing actions

The high-risk boundary is private-key use. The clean architecture is:
- agent submits a transaction intent,
- reference monitor evaluates policy and risk,
- signer service returns signature or denial,
- raw key material never leaves signer boundary.

This follows reference-monitor properties:
- complete mediation,
- tamper resistance,
- verifiability of a small trusted core
[[anderson1972-reference-monitor]]
[[nist-reference-monitor-glossary]].

Design principles:
- least privilege and separation of authority
[[saltzer1975-protection]].

### 3.1 Intent pipeline (kernel-gated)

1. Intent parse: transfer or contract call tuple.
2. Capability check: chain, account, target allowlist, amount/fee bounds.
3. Preflight: estimate cost and bound execution risk.
4. Approval mode: auto for low risk; user or multisig for high risk.
5. Sign path: signer returns signature only.
6. Broadcast: mediated RPC submission and receipt emission.

For Ethereum-style flows:
- fee-policy logic should account for modern fee mechanisms
  [[roughgarden2020-eip1559]].
- keystore handling can align with EIP-2335 format conventions
  [[eip2335-keystore]].

## 4. Role C: receipts with tamper-evident anchoring

Local append-only receipts can be externally anchored without leaking payload:
- compute commitment (hash or Merkle root) over receipt window,
- anchor commitment to public chain or trusted timestamp authority,
- keep sensitive data off-chain by default.

Foundations:
- chained-hash timestamping [[haber1991-timestamp]]
- Merkle batching for scalable proofs [[bayer1993-timestamp]]
- RFC 3161 time-stamp protocol [[rfc3161-tsp]]
- blockchain anchoring pattern [[gipp2015-btc-timestamp]].

## 5. Role D: validator safety and slashing protection

If the system signs consensus messages, the kernel can enforce signing
invariants as a guarded state machine:
- deny equivocation and slashable patterns,
- persist decision state with crash consistency guarantees,
- apply explicit refusal rules before signature release.

For Ethereum consensus clients, this aligns with published slashing-protection
guidance [[ethereum-consensus-validator]].

## 6. Primitive-to-component mapping

Kernel primitives:
- reference monitor (mediated side effects),
- capabilities (explicit authority tokens),
- isolation boundary (sandbox or microVM),
- receipts and commitments (audit plus anchoring).

Blockchain-facing components:
- node P2P runtime constrained by network capability policy,
- chain DB constrained by filesystem capability scope,
- RPC exposure constrained by method allowlist capabilities,
- signer constrained by signing capability predicates,
- validator path constrained by slashing-protection invariants.

## 7. Evaluation protocol suggestions

1. Non-bypass tests for signing path
- prove no signature can be produced outside mediation boundary.

2. Capability attenuation tests
- verify delegated signer rights cannot exceed parent authority.

3. Anchoring integrity tests
- verify commitment proofs for "receipt existed before time T".

4. Privacy leakage tests
- confirm anchored commitments reveal no sensitive payload.

5. Validator safety tests
- replay slashable scenarios and verify refusal invariants.

## 8. Relation to Goni contracts

This document informs (but does not replace):
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/network-gate-and-anonymity.md`
- `blueprint/30-specs/receipts.md`
- `blueprint/30-specs/isolation-and-tool-sandboxes.md`

## 9. Citation keys

See `blueprint/docs/references/bibliography.md`:
- `[[agache2020-firecracker]]`
- `[[watson2010-capsicum]]`
- `[[anderson1972-reference-monitor]]`
- `[[nist-reference-monitor-glossary]]`
- `[[saltzer1975-protection]]`
- `[[roughgarden2020-eip1559]]`
- `[[eip2335-keystore]]`
- `[[haber1991-timestamp]]`
- `[[bayer1993-timestamp]]`
- `[[rfc3161-tsp]]`
- `[[gipp2015-btc-timestamp]]`
- `[[ethereum-consensus-validator]]`
