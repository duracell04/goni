# BOM Experiment - Goni APU Node v1 (Reference Design)

This is an early technical BOM draft for a Goni MVP node based on a high-end APU with unified memory.

## 1. Scope and assumptions

This BOM assumes:
- Single-board compute module with CPU + iGPU + NPU and 128 GB unified LPDDR5X
- Quiet small-form-factor PSU for sustained APU workloads
- Two NVMe SSDs (OS/containers and data/models)
- Mainstream cooling path (air or 240 mm AIO)
- Custom enclosure and front-panel MCU

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

## 3. Component rationale and references

### 3.1 Compute mainboard
- Reference: Framework Desktop Mainboard (Ryzen AI Max class)
- Why: standalone board, unified memory class target, custom integration support
- Link: https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series

### 3.2 Storage split
- OS drive for system/runtime artifacts
- Data/models drive for model weights, embeddings, and user data

### 3.3 Power and cooling
- PSU sized for sustained APU load with transient headroom
- Cooling tuned for low-noise operation in compact enclosure

### 3.4 Enclosure and front panel
- 6-8 L enclosure with filters and serviceable internals
- Front MCU handles power/status signaling to runtime

## 4. Next steps

1. Validate sustained thermal behavior on representative dev hardware.
2. Finalize enclosure + PSU integration for 6-8 L target envelope.
3. Add measured results (thermals, acoustics, sustained throughput) to future BOM revisions.
4. Keep future BOM revisions technical-first until prototype build starts.
