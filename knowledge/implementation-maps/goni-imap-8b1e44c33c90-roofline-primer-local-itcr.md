---
id: GONI-IMAP-8B1E44C33C90
title: Roofline Primer (Local ITCR)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: This appendix provides a concise roofline framing for local inference-time compute reasoning (ITCR).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/appendix/roofline.md
  heading: Roofline Primer (Local ITCR)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Roofline Primer (Local ITCR)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Roofline Primer (Local ITCR)

This appendix provides a concise roofline framing for local inference-time
compute reasoning (ITCR). It defines why decoding is typically memory-bound and
how that shapes platform contracts.
