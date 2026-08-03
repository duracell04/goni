---
id: GONI-IMAP-C5BA1A7C80F7
title: 3. Platform implications for Goni
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Prefer hardware with stable, high sustained bandwidth and predictable latency.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/appendix/roofline.md
  heading: 3. Platform implications for Goni
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Platform implications for Goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Platform implications for Goni

- Prefer hardware with stable, high sustained bandwidth and predictable latency.
- Treat KV cache residency and paging strategy as first-class system concerns.
- Route memory-bound stages to the highest-bandwidth path; avoid unnecessary
  host-device copies.

These are encoded as platform requirements in `blueprint/hardware/10-requirements.md`.
