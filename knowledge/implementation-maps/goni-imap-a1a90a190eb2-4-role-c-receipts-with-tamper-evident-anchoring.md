---
id: GONI-IMAP-A1A90A190EB2
title: '4. Role C: receipts with tamper-evident anchoring'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Local append-only receipts can be externally anchored without leaking payload: compute commitment (hash or Merkle root) over receipt window, anchor commitment to public chain or trusted timestamp authority, keep sensitive data off-chain by default.'
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
  heading: '4. Role C: receipts with tamper-evident anchoring'
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 4. Role C: receipts with tamper-evident anchoring

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
