---
id: GONI-IMAP-30E51C1595CF
title: 3.1 Intent pipeline (kernel-gated)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Intent parse: transfer or contract call tuple.'
domains:
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/45-kernel-blockchain-mapping.md
  heading: 3.1 Intent pipeline (kernel-gated)
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 3.1 Intent pipeline (kernel-gated)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
