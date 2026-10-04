---
id: GONI-IMAP-ED288DB4EAF5
title: Kernel-Blockchain Security Mapping
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Blockchain-facing mechanisms can supply external ordering, shared canonicality, data commitments, or settlement for GONI deployments, while computation validity, data availability, and authority remain separately specified contracts.
domains:
- kernel
- system
- distributed
aliases: []
relations:
- type: depends_on
  target: DIST-01
- type: depends_on
  target: EXEC-01
sources:
- SRC-ETHEREUM-CONSENSUS-VALIDATOR
- SRC-ETHEREUM-DATA-AVAILABILITY
artifacts: []
uncertainty: This is an analytical mapping, not a normative blockchain protocol or an MVP dependency.
legacy:
- path: blueprint/20-system/45-kernel-blockchain-mapping.md
  heading: Kernel-Blockchain Security Mapping
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# Kernel-Blockchain Security Mapping

> Status boundary: this is a specified-only analytical framing, not a claim that GONI uses or requires a blockchain.

A blockchain-facing layer may provide some combination of:

- shared ordering;
- canonical state selection;
- public or consortium commitments;
- payment and settlement;
- dispute or challenge mechanisms;
- Sybil-resistance or validator-selection rules;
- data-availability mechanisms.

The computation substrate may separately provide a result and typed evidence that an accepted relation was satisfied.

These roles must not be collapsed.

[
blockchain-facing\ coordination
\neq
computation\ validity
]

A valid computation proof does not decide which of two competing valid histories becomes canonical. Conversely, consensus over a history does not prove that arbitrary off-chain computation was correct unless the consensus protocol explicitly incorporates the relevant verification rule.

For the local-first GONI MVP, the sovereign kernel remains the canonical authority. Distributed consensus is introduced only where the deployment requires multiple mutually distrustful parties to agree on shared state.
