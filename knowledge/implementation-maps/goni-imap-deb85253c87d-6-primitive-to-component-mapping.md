---
id: GONI-IMAP-DEB85253C87D
title: 6. Primitive-to-component mapping
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Kernel primitives: reference monitor (mediated side effects), capabilities (explicit authority tokens), isolation boundary (sandbox or microVM), receipts and commitments (audit plus anchoring).'
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
  heading: 6. Primitive-to-component mapping
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 6. Primitive-to-component mapping

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
