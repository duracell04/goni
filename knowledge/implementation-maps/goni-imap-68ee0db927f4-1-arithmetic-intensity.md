---
id: GONI-IMAP-68EE0DB927F4
title: 1. Arithmetic intensity
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Arithmetic intensity is: I = FLOPs / Byte.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/appendix/roofline.md
  heading: 1. Arithmetic intensity
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Arithmetic intensity

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Arithmetic intensity

Arithmetic intensity is:

I = FLOPs / Byte.

For a fixed hardware roofline, performance is bounded by the smaller of:

- compute roof: peak FLOPs,
- memory roof: sustained bandwidth * I.

Low-I workloads are memory-bound; high-I workloads are compute-bound.
