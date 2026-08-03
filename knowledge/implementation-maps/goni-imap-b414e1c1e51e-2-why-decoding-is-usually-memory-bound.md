---
id: GONI-IMAP-B414E1C1E51E
title: 2. Why decoding is usually memory-bound
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Autoregressive decoding repeatedly reads model weights and KV cache for each token.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/appendix/roofline.md
  heading: 2. Why decoding is usually memory-bound
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. Why decoding is usually memory-bound

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Why decoding is usually memory-bound

Autoregressive decoding repeatedly reads model weights and KV cache for each
token. The reuse per byte is limited, so I stays low. Prefill/encoding can
exhibit higher I due to larger batch/context reuse, but decode is still dominated
by memory traffic.

Consequence: Increasing TOPS alone does not improve latency unless memory
traffic is reduced or bandwidth increases.
