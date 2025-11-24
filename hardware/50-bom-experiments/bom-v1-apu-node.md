# BOM Experiment - Goni APU Node v1 (Reference Design)

> This document captures **one possible bill-of-materials (BOM)** for a Goni MVP node based on a **high-end APU with unified memory**.
> It is an experiment, not a final decision.
> The goal is to check **technical plausibility** and **order-of-magnitude cost**, using only primary sources (vendor / manufacturer) for specs and list prices where possible.

---

## 1. Scope and assumptions

This BOM assumes:

- A **single-board compute module** with:
  - 16-core CPU,
  - integrated GPU,
  - integrated NPU,
  - **128 GB unified LPDDR5X** memory.
- One **small, quiet PSU** sufficient for ~250 W sustained load.
- Two NVMe SSDs:
  - smaller OS/containers drive,
  - larger data/models drive.
- Standard 120 mm fans and a mainstream CPU cooler (air or 240 mm AIO).
- Custom Goni enclosure and front-panel MCU (power button + status light).

All prices are in **USD**, indicative, and exclude taxes/import duties.

Where prices are taken from official vendor stores, they are noted explicitly. For parts where the manufacturer does not sell directly, prices are left as ranges based on current retail without naming specific retailers.

---

## 2. Summary BOM table (v1 APU Node)

| # | Category           | Example product (reference)                                                                 | Qty | Est. unit price (USD) | Est. ext. (USD) | Notes |
|---|--------------------|---------------------------------------------------------------------------------------------|----:|----------------------:|----------------:|-------|
| 1 | Compute mainboard  | Framework Desktop Mainboard - **Ryzen AI Max+ 395 - 128 GB**                              |  1  | 1,699                 | 1,699           | CPU + iGPU + NPU + 128 GB LPDDR5X, Mini-ITX compatible |
| 2 | OS SSD             | Samsung **990 EVO** 1 TB NVMe SSD                                                          |  1  | ~120                  | ~120            | OS + containers + logs |
| 3 | Data / models SSD  | Samsung **990 EVO Plus** 4 TB Gen4 NVMe SSD                                                |  1  | 299.99                | 299.99          | Models, embeddings, user data |
| 4 | PSU                | 500-600 W SFX/ATX, 80 PLUS Gold (e.g. Corsair SF600)                                       |  1  | ~130-150              | ~130-150        | Quiet, small-form-factor PSU |
| 5 | CPU cooler         | 240 mm AIO or high-end tower cooler (e.g. Corsair iCUE H100x / similar)                    |  1  | ~140-170              | ~140-170        | Keeps APU within TDP under sustained AI load |
| 6 | Case fans          | 2-3x 120 mm PWM fans (e.g. Noctua NF-A12x25 / G2 or equivalent)                            |  3  | ~25-30                | ~75-90          | Quiet airflow for intake + exhaust |
| 7 | NVMe heatsinks     | Simple M.2 heatsinks (if not included with SSDs)                                           |  2  | ~10-15                | ~20-30          | Optional, for thermal headroom |
| 8 | Custom enclosure   | Goni 6-8 L chassis (steel/aluminium, powder-coated, internal brackets)                      |  1  | ~250-350              | ~250-350        | Includes mechanical parts, feet, basic EMI shielding |
| 9 | Front-panel MCU    | Small microcontroller board + LED bar + wiring harness                                     |  1  | ~40-80                | ~40-80          | Handles power button + status LED logic |
|10 | Assembly & QA      | Labour, test time, packaging, small consumables                                            |  1  | ~250-350              | ~250-350        | Per-unit allocation for small batches |

**Indicative total (v1 APU node):**

- **Low end:** ~2,800 USD
- **Midpoint:** ~3,100-3,200 USD
- **High end:** ~3,500 USD

This aligns with the earlier architecture estimate of **~ 2.8-3.5 k USD BOM** for an APU-centric Goni node.

---

## 3. Component rationale and primary references

### 3.1 Compute mainboard (CPU + iGPU + NPU + RAM)

**Example:** Framework Desktop Mainboard - AMD Ryzen AI Max+ 395 - 128 GB

Framework's official marketplace lists the **Ryzen AI Max+ 395 - 128 GB** Desktop mainboard configuration with:

- CPU: 16-core/32-thread AMD Ryzen AI Max+ 395, up to 5.1 GHz, 64 MB L3 cache.
- iGPU: Radeon 8060S Graphics.
- Memory: 128 GB LPDDR5X-8000 (unified).
- Networking: Wi-Fi 7, 5 Gbit Ethernet.
- Form factor: designed to be used in a Desktop chassis with a standard ATX PSU and Mini-ITX-style mounting.
- Price (official store, excl. tax): **US,699** for the 128 GB configuration.

**Why this is a good reference:**

- It exactly matches Goni's **APU-centric design**: powerful CPU + sizeable integrated GPU + NPU + 128 GB unified memory.
- It is sold as a **standalone mainboard**, so Goni can design its own enclosure, PSU, storage layout, and front-panel electronics without reverse-engineering a closed OEM mini-PC.
- It is officially supported for desktop use, not just as a laptop board.

If Framework were not used, a similar Ryzen AI "Strix Halo" mainboard with 128 GB LPDDR5X and Mini-ITX mounts should be chosen.

---

### 3.2 Storage: OS SSD (1 TB) + Data / models SSD (4 TB)

#### 3.2.1 OS drive - Samsung 990 EVO 1 TB

Samsung's **990 EVO** is the current mainstream PCIe 4.0 NVMe SSD line:

- Capacities: 1 TB, 2 TB.
- 2 TB official spec: up to 5,000 MB/s read, 4,200 MB/s write.
- Samsung US store pricing (Oct 2025):
  - 1 TB: US.99
  - 2 TB: US.99

For Goni's OS and container storage, the 1 TB variant is sufficient; the 2 TB variant is a minor cost increase if more headroom for logs and images is desired.

**Chosen reference for BOM:** 1 TB 990 EVO, ~US.

#### 3.2.2 Data/models drive - Samsung 990 EVO Plus 4 TB (Gen4)

Samsung's **990 EVO Plus** is a PCIe 4.0 NVMe SSD offering up to 4 TB capacity. The US product page lists:

- Capacity: up to **4 TB**.
- Sequential read/write up to **7,250 / 6,300 MB/s**.
- US Samsung store price for 4 TB 990 EVO Plus Gen4 NVMe (MZ-V9S4T0B/AM):
  - **US.99**.

4 TB is sufficient to store:

- multiple quantised LLMs (tens of GB each),
- embeddings / vector indices,
- user data (documents, email cache, local knowledge base),
with room for growth.

**Chosen reference for BOM:** 4 TB 990 EVO Plus, US.99.

---

### 3.3 Power supply (PSU) - 500-600 W, SFX/ATX, 80 PLUS Gold

The node's power envelope (APU-centric, no discrete GPU) is expected to stay below ~ 250 W under full AI load. A **500-600 W** supply provides:

- comfortable headroom for transient spikes and future APU revisions,
- higher efficiency (80 PLUS Gold) in the typical 20-50 % load range.

**Example reference:** Corsair SF Series SF600 (600 W SFX, 80 PLUS Gold)

Corsair's official SF600 product page describes:

- 600 W SFX form factor PSU.
- 80 PLUS Gold efficiency (>= 90 % at 50 % load).
- Zero RPM fan mode for silent operation at low/moderate loads.
- 100 % 105 C Japanese capacitors.

The SF600 is widely used in compact high-performance builds and is a suitable reference for Goni's form factor and efficiency requirements. Corsair does not list MSRP on the product page; typical retail prices in late 2025 sit in the **US-150** range depending on region.

**BOM assumption:** generic 500-600 W SFX/ATX Gold PSU, ~US-150.

---

### 3.4 Cooling - CPU cooler and case fans

Goni's APU TDP (Ryzen AI Max+ 395) is around **120 W sustained**, with brief boost above that.

A suitable cooler is:

- either a **240 mm AIO liquid cooler** with low-RPM fans, or
- a high-performance tower air cooler with good acoustics.

**Example reference (AIO):** Corsair iCUE H100x RGB ELITE 240 mm AIO

Corsair's official product page describes the H100x RGB ELITE as a 240 mm radiator AIO with:

- Thermally optimised copper cold plate,
- 240 mm radiator,
- two 120 mm PWM fans,
- 5-year warranty.

Typical European retail pricing is on the order of **EUR 120-160** (~ US-170) depending on region and reseller.

**Example reference (fans):** Noctua NF-A12x25 (120 mm PWM fans)

Noctua's official NF-A12x25 PWM page describes it as a **"high-efficiency quiet 120 mm fan"**, with advanced Sterrox LCP rotor and 0.5 mm tip clearance for high performance at low noise.

For Goni we assume:

- 2x radiator fans (if AIO) plus 1x case exhaust fan, or
- 2-3 case fans if using an air cooler.

**BOM assumption:**

- One 240 mm AIO or equivalent tower cooler: **US-170**.
- Three 120 mm PWM fans: ~US-30 each -> **US-90** total.

---

### 3.5 Enclosure, front MCU, assembly

These are **Goni-specific parts**, not sourced from existing products:

- **Custom enclosure** (6-8 L):
  - steel or aluminium, powder-coated,
  - internal mounts for mainboard, PSU, radiator/fans, SSDs,
  - front and bottom dust filters,
  - clean front panel with button + LED bar.

- **Front-panel MCU + LED bar**:
  - small microcontroller (e.g. ARM Cortex-M or similar) with USB connection to mainboard,
  - handles power button logic and LED states (idle, busy, update, fault).

- **Assembly, QA, packaging**:
  - labour to assemble and test each unit,
  - burn-in and thermal verification,
  - packaging materials.

Initial **per-unit cost allocations** for small production runs might reasonably be:

- Enclosure: **US-350** (low volumes, CNC/proto tooling).
- MCU + LED + harness: **US-80**.
- Assembly & QA: **US-350**.

These figures should be refined as mechanical design and manufacturing quotes are obtained.

---

## 4. BOM conclusions and next steps

From this experiment:

- A **realistic APU-based Goni node** can be built with a **BOM in the ~ US.8-3.5k range**, while meeting the:

  - size constraint (compact 6-8 L enclosure),
  - performance target (30-40B quantised LLMs locally),
  - noise and power constraints (few hundred watts, quiet cooling).

- The single most expensive part is the **compute mainboard with 128 GB LPDDR5X** (approx US,699 in the Framework Desktop configuration).

- All other components (storage, PSU, cooling, enclosure, assembly) combined add roughly another **US.1-1.8k**.

Given a target hardware selling price of **US,000 per node**, this leaves substantial margin to:

- cover non-recurring engineering (NRE) and tooling,
- handle small batch manufacturing costs and warranty risk,
- subsidise early software development and support.

**Next steps for hardware team:**

1. Validate that the proposed power envelope and cooling strategy are sufficient for sustained AI workloads on an APU node (thermal tests on dev hardware).
2. Refine the enclosure + PSU integration for the 6-8 L form factor.
3. Obtain preliminary manufacturing quotes for the enclosure and assembly to convert the **rough cost ranges** into more precise ranges.
4. When the mainboard vendor and exact cooler/PSU vendors are locked in, clone this file as om-vX-<vendor>-apu-node.md with specific part numbers and updated prices.

---

## 5. Primary references

**Compute mainboard**

- Framework Desktop Mainboard - AMD Ryzen AI Max 300 Series
  https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series

**Storage**

- Samsung 990 EVO NVMe SSD (product family overview)
  https://semiconductor.samsung.com/consumer-storage/internal-ssd/990-evo/
- Samsung 990 EVO 2 TB NVMe SSD - US store (shows 1 TB / 2 TB capacities and prices)
  https://www.samsung.com/us/computing/memory-storage/solid-state-drives/990-evo-nvme-ssd-2tb-mz-v9e2t0b-am/
- Samsung 990 EVO Plus NVMe SSD (product family overview)
  https://semiconductor.samsung.com/consumer-storage/internal-ssd/990-evo-plus/
- Samsung 990 EVO Plus Gen4 NVMe SSD 4 TB - US store
  https://www.samsung.com/us/computing/memory-storage/solid-state-drives/990-evo-plus-gen4-nvme-ssd-4tb-mz-v9s4t0b-am/

**Power supply**

- Corsair SF Series SF600 - 600 W 80 PLUS Gold SFX PSU
  https://www.corsair.com/us/en/p/psu/cp-9020105-na/sf-series-sf600-600-watt-80-plus-gold-certified-high-performance-sfx-psu-cp-9020105-na

**Cooling**

- Corsair iCUE H100x RGB ELITE Liquid CPU Cooler (240 mm AIO)
  https://www.corsair.com/ch/en/p/cpu-coolers/cw-9060065-ww2/icue-h100x-rgb-elite-liquid-cpu-cooler-cw-9060065-ww2
- Noctua NF-A12x25 PWM - 120 mm PWM case fan
  https://www.noctua.at/en/products/nf-a12x25-pwm
