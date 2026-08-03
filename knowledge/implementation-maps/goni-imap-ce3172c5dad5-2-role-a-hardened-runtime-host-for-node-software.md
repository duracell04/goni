---
id: GONI-IMAP-CE3172C5DAD5
title: '2. Role A: hardened runtime host for node software'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'When running a blockchain node, the kernel contribution is compartmentalization plus resource governance: isolate node process and storage authority, constrain egress and RPC exposure, budget CPU, IO, and network to protect interactive QoS.'
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
  heading: '2. Role A: hardened runtime host for node software'
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 2. Role A: hardened runtime host for node software

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
