---
id: GONI-SYNTHESIS-71087555FF15
title: 2.2 Hardware constraints
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Form factor**: Small, unobtrusive box: target **6-8 L volume** (e.g.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 2.2 Hardware constraints
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 2.2 Hardware constraints

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Hardware constraints

- **Form factor**:
  - Small, unobtrusive box: target **6-8 L volume**  
    (e.g. roughly 28 x 22 x 13 cm, subject to mechanical design)
  - Matte black, no RGB, one discreet light bar + power button.

- **Noise**:
  - Under normal interactive use (chat, coding): subjectively **"silent"** at desk distance.  
  - Under sustained heavy load: quieter than a typical gaming PC or PS5.

- **Compute platform (MVP assumption)**:
  - APU-first, GPU-optional.
  - Baseline: **AMD Ryzen AI Max+ 395** (Strix Halo) class APU  
    - 16 cores / 32 threads, up to 5.1 GHz  
    - Radeon 8060S iGPU (RDNA 3.5, ~40 CUs)  
    - XDNA 2 NPU (~50+ TOPS)  
  - **128 GB LPDDR5X** unified memory (soldered, no user RAM upgrades).

- **Storage**:
  - `>= 1 TB` NVMe Gen4 for OS and containers.
  - `>= 4 TB` NVMe Gen4/5 for models, embeddings, and user data.
  - At least **one spare M.2 slot** reserved for future expansion (extra SSD or M.2 accelerator).

- **Networking**:
  - At least **2.5G Ethernet**, but design as if **10G** is normal (for mesh).  
  - Wi-Fi 7 + BT for convenience, not core.

- **Power**:
  - **500-600 W 80+ Gold** PSU (SFX or compact ATX).  
  - Target sustained power under load: **200-250 W** per node (APU + SSDs + fans).
