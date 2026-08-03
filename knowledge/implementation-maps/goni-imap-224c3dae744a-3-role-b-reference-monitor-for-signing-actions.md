---
id: GONI-IMAP-224C3DAE744A
title: '3. Role B: reference monitor for signing actions'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The high-risk boundary is private-key use.
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
  heading: '3. Role B: reference monitor for signing actions'
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 3. Role B: reference monitor for signing actions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
