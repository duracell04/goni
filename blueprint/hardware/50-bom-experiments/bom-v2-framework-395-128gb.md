# BOM Experiment - Goni APU Node v2 (Ryzen AI Max+ 395 / 128 GB)

Snapshot date: **2025-12-14**  
Scope: **MVP reference** APU-centric node in a ~7 L enclosure

This BOM is a refreshed version of the earlier v1 experiment. It fixes formatting issues, updates major prices from current listings, and makes explicit what is still an estimate.

---

## 1. Scope and assumptions

This BOM assumes:

- A single compute mainboard with:
  - AMD Ryzen AI Max+ 395 class APU (CPU + iGPU + NPU),
  - **128 GB unified LPDDR5X** (soldered),
  - Mini-ITX-style mounting and standard ATX/SFX PSU compatibility.
- Internal PSU (quiet), two NVMe drives, and a simple front-panel controller.
- Custom 6–8 L enclosure; MVP target ~7 L.

Currency rules:

- **Major anchor prices** (mainboard / off-the-shelf fallback) are taken from public listings on the snapshot date.
- Everything else is a **range** (region-dependent, supplier-dependent) and should be re-quoted when we approach manufacturing.

---

## 2. Anchor price references (current reality)

### 2.1 Primary compute module (board-based)

Framework Marketplace lists:

- Ryzen AI Max+ 395 – **128 GB**: **$1,699** (excl. tax)  
  Link: https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series?v=FRAFMK0002

This is the **primary reference** because it is a standalone mainboard and supports “board swap” upgrades.

### 2.2 Off-the-shelf fallback dev box

HP Switzerland lists the Z2 Mini G1a configuration (Ryzen AI Max+ PRO 395, 128 GB, 1 TB SSD) at:

- **CHF 3'299 incl. VAT**  
  Link: https://www.hp.com/ch-en/shop/products/desktops/hp-z2-mini-g1a-workstation-desktop-pc-a40q4et-uuz

Aggregators in Switzerland show similar configs around:

- **~CHF 2'883** (varies by reseller and options)  
  Link: https://www.toppreise.ch/preisvergleich/Server-Workstations/HP-Z2-Mini-G1a-Workstation-AMD-Ryzen-AI-Max-PRO-395-A40Q4ET-p816469

---

## 3. Summary BOM table (DIY board-based Goni node)

| # | Category | Example (reference) | Qty | Est. unit price | Est. ext. | Notes |
|---|----------|----------------------|----:|----------------:|----------:|-------|
| 1 | Compute mainboard | Framework Desktop Mainboard – Ryzen AI Max+ 395 – 128 GB | 1 | **$1,699** | **$1,699** | Anchor price (snapshot) |
| 2 | OS SSD | 2 TB NVMe PCIe 4.0 | 1 | $140–220 | $140–220 | OS, containers, logs |
| 3 | Data/models SSD | 4 TB NVMe PCIe 4.0 | 1 | $220–450 | $220–450 | Models, embeddings, user data |
| 4 | PSU | 500–600 W SFX, 80+ Gold, ATX 3.x preferred | 1 | $120–220 | $120–220 | Quiet + efficient, headroom |
| 5 | Cooling | 240 mm AIO **or** high-end air tower + 2–3 PWM fans | 1 | $120–240 | $120–240 | Tune for acoustics (low RPM) |
| 6 | Enclosure | Custom 6–8 L chassis + brackets + filters | 1 | $200–450 | $200–450 | Prototype vs small-batch differ a lot |
| 7 | Front panel | MCU + status bar + harness | 1 | $30–120 | $30–120 | USB HID or serial; no RGB effects |
| 8 | Assembly & QA | Labour + burn-in + packaging | 1 | $200–450 | $200–450 | Per-unit allocation (small batches) |

**Indicative total (DIY board-based):**  
- Low: **~$2,829**  
- Mid: **~$3,300**  
- High: **~$3,849**

What to take away:

- The compute mainboard is still the dominant cost driver.
- The rest is mostly “box-building economics” (enclosure + assembly + quality).

---

## 4. Alternative: “buy a dev box first” (HP Z2 Mini G1a)

If the goal is **software validation now** (rather than custom enclosure work), the HP Z2 Mini G1a is a reasonable “dev box” path:

- You get the 128 GB unified-memory class immediately,
- thermals/acoustics are OEM-solved,
- but you don’t learn the enclosure/PSU/front-panel integration details.

This is a **dev/prototyping fallback**, not the product hardware story.

---

## 5. Next BOM steps

1. Create **bom-v3** once the enclosure layout is pinned (fan sizes, PSU placement, exact NVMe count).
2. Add a second column for **CH/EU street prices** if we want a Europe-first manufacturing plan.
3. When the runtime backend is decided (vLLM ROCm vs llama.cpp), add “software constraint notes” per line item (OS choice, driver stack, power budget).
