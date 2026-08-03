---
id: GONI-IMAP-14E9D52FD8F6
title: 1.2.3 Memory topology
type: implementation-map
status: draft
implementation_state: specified_only
proposition: UMA reduces copies but shares bandwidth; a discrete GPU provides dedicated VRAM bandwidth but incurs PCIe transfer and wake penalties.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.2.3 Memory topology
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.2.3 Memory topology

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2.3 Memory topology

UMA reduces copies but shares bandwidth; a discrete GPU provides dedicated VRAM
bandwidth but incurs PCIe transfer and wake penalties.

Normative requirement:
- UMA is preferred for high-frequency state exchange.
- dGPU is acceptable only if state shuttling over PCIe is avoided.
