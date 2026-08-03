---
id: GONI-IMAP-165DEF43EF52
title: 1.2.1 Memory wall (roofline framing)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Define arithmetic intensity as: I = FLOPs / Byte.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.2.1 Memory wall (roofline framing)
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.2.1 Memory wall (roofline framing)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2.1 Memory wall (roofline framing)

Define arithmetic intensity as:

I = FLOPs / Byte.

Decoding/generation tends to be low-I and thus memory-bound; prefill/encoding
can be more compute-bound. As a result, platform selection MUST prioritize
sustained bandwidth, latency stability, and memory residency over peak TOPS.
See `blueprint/hardware/appendix/roofline.md` for the roofline primer.
