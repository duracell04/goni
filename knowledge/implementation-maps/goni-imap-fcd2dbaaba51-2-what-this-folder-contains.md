---
id: GONI-IMAP-FCD2DBAABA51
title: 2. What this folder contains
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 10-requirements.md The primary reference for **hardware requirements** (size, power, connectivity, noise, serviceability, future-proofing).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 2. What this folder contains
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 2. What this folder contains

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. What this folder contains

- [`10-requirements.md`](/blueprint/hardware/10-requirements.md)  
  The primary reference for **hardware requirements** (size, power, connectivity, noise, serviceability, future-proofing).

- [`20-architecture-options.md`](/blueprint/hardware/20-architecture-options.md)  
  Updated survey of **candidate architectures and concrete 2025/2026 SKUs**, with an explicit MVP recommendation and resolved open questions.

- [`25-hardware-layers-and-supplier-map.md`](/blueprint/hardware/25-hardware-layers-and-supplier-map.md)  
  Opinionated map of **accelerator/supplier landscape**, **availability**, and how it aligns to Goni tiers and current software backend readiness.
- [`os-and-base-image.md`](/blueprint/hardware/os-and-base-image.md)  
  OS-level telemetry and capability discovery contract for the base image.

- [`30-mechanical/`](/blueprint/hardware/30-mechanical)  
  Enclosure concepts, airflow notes, and draft thermal + acoustic plans.

- [`40-electronics/`](/blueprint/hardware/40-electronics)  
  Power distribution assumptions, PSU choices, front-panel MCU, LED/status bar, harnessing.

- [`50-bom-experiments/`](/blueprint/hardware/50-bom-experiments)  
  Bill-of-materials experiments and component snapshots. New versions should be added rather than overwriting older ones.

- [`90-decisions.md`](/blueprint/hardware/90-decisions.md)  
  Accepted hardware decisions (ADR-style): baseline architecture, networking, enclosure envelope, PSU approach, etc.
- [`appendix/roofline.md`](/blueprint/hardware/appendix/roofline.md)  
  Roofline primer used by ITCR platform contracts.

---
