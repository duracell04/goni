---
id: GONI-IMAP-1DE64C8F3A3F
title: 2. Summary BOM table (v1 technical draft)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '| # | Category | Example product (reference) | Qty | Notes | | 1 | Compute mainboard | Framework Desktop Mainboard - Ryzen AI Max+ 395 - 128 GB | 1 | CPU + iGPU + NPU + unified memory | | 2 | OS SSD | 1-2 TB NVMe SSD | 1 | OS + containers + logs | | 3 | Data/models SSD | 4 TB NVMe SSD | 1 | Models, embeddings, user data |'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/50-bom-experiments/bom-v1-apu-node.md
  heading: 2. Summary BOM table (v1 technical draft)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2. Summary BOM table (v1 technical draft)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Summary BOM table (v1 technical draft)

| # | Category | Example product (reference) | Qty | Notes |
|---|----------|------------------------------|----:|-------|
| 1 | Compute mainboard | Framework Desktop Mainboard - Ryzen AI Max+ 395 - 128 GB | 1 | CPU + iGPU + NPU + unified memory |
| 2 | OS SSD | 1-2 TB NVMe SSD | 1 | OS + containers + logs |
| 3 | Data/models SSD | 4 TB NVMe SSD | 1 | Models, embeddings, user data |
| 4 | PSU | 500-600 W SFX/ATX, 80+ Gold | 1 | Quiet, compact PSU path |
| 5 | CPU cooler | 240 mm AIO or high-end tower cooler | 1 | Sustained load thermals |
| 6 | Case fans | 2-3 x 120 mm PWM fans | 3 | Intake + exhaust airflow |
| 7 | NVMe heatsinks | M.2 heatsinks (if needed) | 2 | Thermal headroom |
| 8 | Custom enclosure | Goni 6-8 L chassis | 1 | Mechanical integration |
| 9 | Front-panel MCU | Small MCU + LED bar + harness | 1 | Button/status logic |
|10 | Assembly and QA | Labor + burn-in + packaging | 1 | Small-batch validation |
