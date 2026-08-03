---
id: GONI-IMAP-D9D9D7627297
title: '5. Role D: validator safety and slashing protection'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'If the system signs consensus messages, the kernel can enforce signing invariants as a guarded state machine: deny equivocation and slashable patterns, persist decision state with crash consistency guarantees, apply explicit refusal rules before signature release.'
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
  heading: '5. Role D: validator safety and slashing protection'
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 5. Role D: validator safety and slashing protection

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Role D: validator safety and slashing protection

If the system signs consensus messages, the kernel can enforce signing
invariants as a guarded state machine:
- deny equivocation and slashable patterns,
- persist decision state with crash consistency guarantees,
- apply explicit refusal rules before signature release.

For Ethereum consensus clients, this aligns with published slashing-protection
guidance [[ethereum-consensus-validator]].
