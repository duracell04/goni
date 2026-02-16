# BOM Experiment - Goni APU Node v2 (Ryzen AI Max+ 395 / 128 GB)

Snapshot date: 2025-12-14
Scope: MVP reference APU-centric node in a ~7 L enclosure.

This BOM is a technical integration snapshot for the hardware path.

## 1. Scope and assumptions

This BOM assumes:
- A single compute mainboard with:
  - AMD Ryzen AI Max+ 395 class APU (CPU + iGPU + NPU)
  - 128 GB unified LPDDR5X
  - Mini-ITX-style mounting and standard ATX/SFX PSU compatibility
- Internal PSU (quiet), two NVMe drives, and a simple front-panel controller
- Custom 6-8 L enclosure; MVP target ~7 L

## 2. Reference hardware set

### 2.1 Primary compute module (board-based)
- Framework Desktop Mainboard, Ryzen AI Max+ 395, 128 GB unified memory
- Link: https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series?v=FRAFMK0002

Why it matters:
- standalone board for custom enclosure and front-panel integration
- direct board-swap path for future APU generations

### 2.2 Off-the-shelf fallback dev box
- HP Z2 Mini G1a class configuration with Ryzen AI Max+ PRO 395 and 128 GB
- Link: https://www.hp.com/ch-en/shop/products/desktops/hp-z2-mini-g1a-workstation-desktop-pc-a40q4et-uuz

Why it matters:
- immediate software validation path
- verifies compact unified-memory workstation class as fallback

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

## 4. Alternative path: dev box first

If the goal is software validation now (without enclosure work), use an OEM dev box path first, then migrate to board-based integration once runtime and thermal contracts stabilize.

## 5. Next steps

1. Create bom-v3 once enclosure layout is fixed (fan sizes, PSU placement, NVMe count).
2. Add per-line software constraints (backend/driver assumptions, power envelopes, thermal envelopes).
3. Add measured thermal/acoustic results for each candidate layout.
