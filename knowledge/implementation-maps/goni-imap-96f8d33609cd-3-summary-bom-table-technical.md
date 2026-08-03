---
id: GONI-IMAP-96F8D33609CD
title: 3. Summary BOM table (technical)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '| # | Category | Example (reference) | Qty | Notes | | 1 | Compute mainboard | Framework Desktop Mainboard - Ryzen AI Max+ 395 - 128 GB | 1 | Primary integration target | | 2 | OS SSD | 2 TB NVMe PCIe 4.0 | 1 | OS, containers, logs | | 3 | Data/models SSD | 4 TB NVMe PCIe 4.0 | 1 | Models, embeddings, user data | | 4 | PSU | 500-600 W SFX, 80+ Gold, ATX 3.x preferred | 1 | Quiet + headroom |'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/50-bom-experiments/bom-v2-framework-395-128gb.md
  heading: 3. Summary BOM table (technical)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3. Summary BOM table (technical)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Summary BOM table (technical)

| # | Category | Example (reference) | Qty | Notes |
|---|----------|----------------------|----:|-------|
| 1 | Compute mainboard | Framework Desktop Mainboard - Ryzen AI Max+ 395 - 128 GB | 1 | Primary integration target |
| 2 | OS SSD | 2 TB NVMe PCIe 4.0 | 1 | OS, containers, logs |
| 3 | Data/models SSD | 4 TB NVMe PCIe 4.0 | 1 | Models, embeddings, user data |
| 4 | PSU | 500-600 W SFX, 80+ Gold, ATX 3.x preferred | 1 | Quiet + headroom |
| 5 | Cooling | 240 mm AIO or high-end air tower + 2-3 PWM fans | 1 | Tune for acoustics |
| 6 | Enclosure | Custom 6-8 L chassis + brackets + filters | 1 | Prototype integration |
| 7 | Front panel | MCU + status bar + harness | 1 | USB HID or serial |
| 8 | Assembly and QA | Burn-in and thermal validation | 1 | Small-batch process |
