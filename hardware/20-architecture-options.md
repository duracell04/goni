# 20 - Candidate Hardware Architectures for Goni (MVP)

This document surveys the **hardware architecture options** for Goni MVP, based on:

- the requirements in [10-requirements.md](./10-requirements.md),
- example products already on the market (GMKtec, HP, Framework, Acer),
- and an explicit comparison of trade-offs (size, power, performance, cost, future-proofing).

The goal is to stay **technology-agnostic at the level of Goni** (no single vendor lock-in), while using concrete products as **reference points** for what is technically and economically realistic in 2025-2027.

---

## 1. Recap of Goni hardware requirements

From 10-requirements.md, the Goni node must:

- Be a **small, quiet appliance** (~6-8 L volume), suitable for desk or shelf.
- Run **medium-to-large language models** (on the order of 30-40B parameters when quantised) with interactive latency.
- Handle **retrieval-augmented generation (RAG)** over personal data and light adapter / LoRA training.
- Operate from a **household power outlet**, with typical power draw in the **few hundred watts** range at most.
- Act as a **cluster node**, such that 2-4 boxes on a normal Ethernet network behave like one logical AI system.
- Remain **future-proof**: main compute boards should be upgradable without redesigning the entire enclosure and power system.

Given this, we consider three main architectures:

1. APU-centric node (Ryzen AI-class integrated CPU/GPU/NPU),
2. Discrete GPU workstation (desktop CPU + powerful dGPU),
3. External Blackwell node (Grace Blackwell GN100) as a heavy add-on.

---

## 2. Architecture A - APU-centric node (Ryzen AI Max+ 395 class)

### 2.1 Concept

The first architecture uses a **high-end APU**: CPU + GPU + NPU on one package, with **unified LPDDR5X memory**.

The most concrete examples today are:

- GMKtec EVO-X2 AMD Ryzen AI Max+ 395 Mini PC
  - AMD Ryzen AI Max+ 395: 16 cores / 32 threads, up to 5.1 GHz, 80 MB cache
  - XDNA 2 NPU, "126 TOPS total" AI performance
  - Radeon 8060S iGPU (40 RDNA 3.5 compute units)
  - 64 or 128 GB LPDDR5X-8000, up to 16 TB SSD (dual M.2)
  - Price: from **EUR 1,499** (~US.6k) for 64 GB / 1 TB, more for 128 GB / 2 TB.
  [GMKtec product page](https://de.gmktec.com/en/products/gmktec-evo-x2-amd-ryzen%E2%84%A2-ai-max-395-mini-pc-1)
  [GMKtec EVO-X2 on Amazon](https://www.amazon.com/GMKtec-ryzen_ai_mini_pc_evo_x2/dp/B0F53MLYQ6)

- HP Z2 Mini G1a Workstation Desktop PC - AI PC
  - AMD Ryzen AI Max+ PRO 395 (up to 5.1 GHz, 16c/32t, 64 MB L3 cache)
  - Integrated Radeon 8060S Graphics
  - Up to **128 GB unified LPDDR5X-8533** memory
    - HP explicitly states that up to **96 GB** can be assigned exclusively to the GPU, "rivaling the memory in two high-end graphics cards".
  - Up to 8 TB NVMe SSD (dual modules, RAID optional)
  - Up to 50 NPU TOPS
  - Price: typical 64 GB / 2 TB configs around **EUR 2,299** (~US.5-2.7k) in Europe.
  [HP Z2 Mini G1a official page](https://www.hp.com/us-en/workstations/z2-mini-a.html)
  [HP Z2 Mini G1a 64 GB / 2 TB example listing](https://shop.bechtle-plm.com/produkt/cad-workstations/hp-z2-mini-g1a-amd-ryzen-ai/)

- Framework Desktop mainboard (AMD Ryzen AI Max 300 series)
  - Ryzen AI Max+ 395:
    - 16-core / 32-thread Zen 5 CPU
    - Up to 5.1 GHz boost
    - 64 MB L3 cache
    - Radeon 8060S Graphics (40 RDNA 3.5 CUs)
    - Up to 128 GB LPDDR5X-8000 unified memory
  - Networking: Wi-Fi 7, 5 Gbit Ethernet
  - The mainboard is sold **stand-alone** and fits a Mini-ITX mounting pattern, designed to work with standard ATX PSUs and cases.
  - Price (official, excluding tax):
    - Max+ 395 - 64 GB: **US,299**
    - Max+ 395 - 128 GB: **US,699**
  [Framework Desktop mainboard product page](https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series)
  [Framework Desktop overview](https://frame.work/desktop)

### 2.2 Behaviour vs Goni's needs

**Strengths:**

- **Unified memory**: up to 128 GB LPDDR5X, with up to 96 GB assignable to the GPU (HP Z2). This is exceptionally attractive for running **30-40B parameter LLMs** in 4-bit or 8-bit quantised formats and still having headroom for embeddings and caches. HP's positioning explicitly calls out running large AI workloads locally in this configuration.

- **Balanced compute**:
  - 16 Zen 5 cores are plenty for orchestrating RAG pipelines, serving multiple users, and handling background jobs.
  - The integrated Radeon 8060S iGPU with 40 CUs and fast memory is strong for AI inference and moderate graphics.
  - The XDNA 2 NPU provides tens of TOPS for smaller models and pre/post-processing tasks.

- **Power and thermals**:
  - AMD and Framework indicate sustained power levels around **120 W** for Ryzen AI Max+ 395 in desktop form factors, with short boosts higher. This implies total system power for a Goni node in the **200-250 W** range under load, which is very manageable in a 6-8 L enclosure with conventional cooling.

- **Size**:
  - GMKtec and HP both demonstrate that such an APU can be packaged in extremely compact mini-PCs. Framework's own Desktop is a 4.5 L design. A 6-8 L Goni box has generous room for a quiet PSU, multiple SSDs, and larger fans.

- **Ecosystem and openness**:
  - Using a board like Framework's means we can combine a **known-good compute module** with our own industrial design, PSU, storage, and front-panel logic, without reverse-engineering a closed OEM mini-PC.

**Weaknesses / risks:**

- **Soldered memory**: 128 GB LPDDR5X is not user-upgradable. The choice of memory capacity is a factory decision and must be correct for the product's lifetime.

- **No ECC** on LPDDR5X in these consumer devices (as of 2025). For Goni's domain (personal / small-team assistant), this is acceptable but should be acknowledged.

- **Vendor/platform longevity**:
  - Third-party reviews (e.g. PC Gamer, The Verge) note that while Framework's Desktop is powerful and well-suited for AI workloads, the long-term roadmap for Strix Halo-class APUs is not yet as mature as mainstream desktop platforms. This is a general ecosystem risk, not a show-stopper.
  [Framework Desktop review - PC Gamer](https://www.pcgamer.com/hardware/gaming-pcs/framework-desktop-pc-review/)
  [Framework Desktop review - The Verge](https://www.theverge.com/reviews/749404/framework-desktop-pc-amd-ryzen-ai-max-385-395-plus-review)

**Fit for Goni MVP:**

- Aligns extremely well with:
  - small, quiet appliance form factor,
  - 30-40B quantised LLM inference,
  - local-first RAG + adapters,
  - power envelope < 300 W,
  - future upgradability by swapping the mainboard for next-gen APU and keeping the enclosure + PSU.

This is the **leading candidate architecture** for Goni MVP.

---

## 3. Architecture B - Discrete GPU workstation (desktop CPU + dGPU)

### 3.1 Concept

The traditional "AI workstation" architecture uses:

- Desktop CPU (e.g. Ryzen 9 or Core i9),
- High-end discrete GPU(s) (e.g. NVIDIA RTX 5090),
- Large DDR5 DIMM pool (128-256 GB),
- 24-48 GB GDDR7 VRAM per GPU,
- 1000-1600 W PSU.

Such systems are well-known in ML research and enterprise environments; they can match or exceed a single H100 on some LLM inference workloads, especially with 2x top-end GPUs.

### 3.2 Behaviour vs Goni's needs

**Strengths:**

- **Raw GPU horsepower**:
  - Massive FP16/INT8/FP8 FLOPS, ideal for high-throughput inference and training.
  - Large dedicated VRAM (24-48 GB per card) gives comfortable headroom for 70B+ models without heavy quantisation.

- **Mature ecosystem**:
  - CUDA, cuDNN, TensorRT, and a vast ecosystem of tooling around discrete GPUs.

- **ECC options** if based on workstation cards and server memory.

**Weaknesses for Goni MVP:**

- **Form factor and acoustics**:
  - A system with a high-end GPU tends to be physically larger (full ATX towers) and louder under load.
  - Making it truly quiet in <= 8 L is extremely challenging; most such systems end up >10-15 L to allow for enough airflow and large radiators.

- **Power**:
  - Single top-end GPUs now commonly have **board power in the 400-600 W range**. Together with CPU and other components, total system power can approach or exceed **700-900 W**.
  - This is far out of line with a "small domestic appliance" expectation.

- **Overkill for MVP use-case**:
  - Goni does not target local full-model training of 70B+ models.
  - Inference and adapters on 30-40B models can be done more efficiently with an APU architecture.

**Fit for Goni:**

- This architecture is more suitable for a **future "Goni Lab / Tower"** product aimed at ML developers who explicitly want a desktop training box.
- It is **not** aligned with the Goni MVP goals (small, quiet, appliance-like AI node).

---

## 4. Architecture C - External Grace Blackwell node (Acer Veriton GN100)

### 4.1 Concept

The third architecture is to **keep Goni node small and APU-based**, but allow attaching an external **mini supercomputer** node for heavy workloads.

The clearest example is the **Acer Veriton GN100 AI Mini Workstation**, described by Acer and NVIDIA as a third-party implementation of NVIDIA's "Project Digits" mini-supercomputer:

- **NVIDIA GB10 Grace Blackwell Superchip**:
  - 20 Arm CPU cores
  - Blackwell GPU with fifth-gen Tensor Cores
  - 128 GB LPDDR5X "coherent unified system memory"
  - Up to **1 petaFLOP of FP4 AI performance**
- Storage: up to 4 TB M.2 NVMe (self-encrypting).
- Connectivity: Ethernet, Wi-Fi 7, USB, HDMI, and a **ConnectX-7** NIC allowing **dual-node operation**, claimed to handle models up to ~405B parameters when two units are linked.
- Physical size: **150 x 150 x 50.5 mm**, <1.5 kg.
- Price: starting at **US,999** in North America (and EUR 3,999 in EMEA).

[Acer GN100 official product page](https://www.acer.com/us-en/desktops-and-all-in-ones/veriton-workstations/veriton-gn100-ai-mini-workstation)
[Acer GN100 press release](https://news.acer.com/acer-unveils-the-veriton-gn100-ai-mini-workstation-built-on-the-nvidia-gb10-superchip)
[Tom's Hardware coverage](https://www.tomshardware.com/tech-industry/artificial-intelligence/acer-unveils-project-digits-supercomputer-featuring-nvidias-gb10-superchip-with-128gb-of-lpddr5x)
[TechRadar Pro coverage](https://www.techradar.com/pro/acer-joins-nvidia-gb10-superchip-fan-club-with-usd3-999-veriton-ai-mini-workstation-pc-and-yes-you-can-link-two-to-get-even-more-power)

### 4.2 Behaviour vs Goni's needs

**Strengths:**

- **Genuinely "mini-DGX"**: 128 GB unified memory + 1 PFLOP FP4 is overkill for a single-user Goni node, but ideal for heavy experimentation, pretraining, or large-scale inference.

- **Compact**: 0.5 L box that can sit next to the Goni node(s), not a rack.

- **Software stack**: ships with NVIDIA DGX Base OS, CUDA, PyTorch, Jupyter, etc., ready for LLM workloads.

**Weaknesses / integration challenges:**

- **Arm architecture**: host CPU is Arm-based, not x86; this is fine if used as a remote server, but different from Goni's base.

- **Vendor stack**: tightly integrated in NVIDIA's ecosystem (DGX OS, license terms, and update policies).

- **Cost**: starting at ,999 for base config; two units for dual-node "mini cluster" -> ,000+.

**Fit for Goni:**

- Very attractive as a **future "Goni Max / Enterprise" add-on node** that the Goni mesh can offload heavy jobs to via Ethernet.
- Not essential for the **first Goni MVP** node, which should work well with APU-only hardware.

---

## 5. Comparative summary

### 5.1 Qualitative comparison

| Aspect                        | APU-centric (Ryzen AI Max+ 395) | Discrete GPU Workstation          | Grace Blackwell GN100 (external)       |
|------------------------------|----------------------------------|-----------------------------------|----------------------------------------|
| Node size                    | 4-8 L possible                  | 10-20+ L typical                  | 0.5 L (standalone)                     |
| Power draw (per node)        | ~200-250 W                      | 700-900 W possible                | unspecified, but high for 1 PFLOP      |
| Local LLM size (quantised)   | 30-40B comfortable, more with trade-offs | 70B+ with high throughput        | 70-400B (paired)                       |
| Memory for AI                | 128 GB unified LPDDR5X          | 24-48 GB VRAM + 128-256 GB system | 128 GB unified LPDDR5X                 |
| Noise / acoustics            | Quiet with standard cooling     | Harder to keep quiet in small box | Vendor-tuned, advertised as desktop    |
| Complexity (hardware)        | Low - single APU board          | High - CPU + motherboard + dGPU   | External unit, separate management     |
| Cost for base compute module | ~ .3-1.7k (APU board)        | -4k+ for GPU alone             | ,999+                               |
| Fit to Goni MVP              | **Very good**                   | Overkill / misaligned             | **Excellent add-on, not base**         |

### 5.2 Goni MVP baseline recommendation

Given the above:

- **Baseline Goni node (MVP)** should be **APU-centric**:
  - high-end Ryzen AI-class APU with integrated GPU and NPU,
  - 128 GB unified LPDDR5X memory,
  - multi-terabyte NVMe storage,
  - multi-gigabit Ethernet.

- The most practical way to do this **without becoming a Mini-PC OEM** is to:

  1. Use a compute module similar to the **Framework Desktop mainboard, AMD Ryzen AI Max+ 395 - 128GB**:
     - official part, US,699 pre-tax as of late 2025.
     - vendor: Framework.

  2. Integrate it into a **custom Goni enclosure** with:
     - 500-600 W PSU,
     - 1 + 4 TB NVMe,
     - quiet fans + front MCU,
     - status LEDs and power management.

- **Future tiers**:
  - **Goni Max Tower**: desktop CPU + discrete GPU tower for users who explicitly want local heavy training.
  - **Goni Max Node (GN100)**: mesh integration with Acer Veriton GN100-class devices as external heavy nodes, using them as remote accelerators while the Goni APU node remains the "control" and data host.

---

## 6. Open questions / items for discussion

1. What is the **minimum acceptable memory capacity** for APU-based Goni (is 64 GB ever acceptable, or is 128 GB mandatory for MVP)?
2. Do we want to **standardise on a specific mechanical envelope** (e.g. 7 L target volume) or leave some flexibility (6-8 L)?
3. Should the MVP node ship with **multi-gigabit Ethernet as standard** (e.g. 5 GbE / 10 GbE), or is 2.5 GbE sufficient for the first hardware revision?
4. How strongly do we want to **tie ourselves to Framework's board** vs. designing for "any Strix Halo / Ryzen AI board in Mini-ITX form factor"?
5. At what point do we officially define a **"Goni Max / GN100 integration"** story in hardware, vs. leaving that as a software-only extension?

---

## 7. Market scan: host candidates and DePIN networks

Quick map of "where Goni OS could land" across near-term hardware and decentralized compute. Use this to align industrial design, performance expectations, and potential hybrid/offload paths.

### 7.1 Local "AI boxes" / workstations (tinybox-style)
- tinybox (tinygrad / tinycorp) — open hardware/software positioning for small AI training/inference boxes. Docs: https://docs.tinygrad.org/tinybox/ | Overview: https://tinygrad.org/#tinybox
- NVIDIA DGX Spark — compact NVIDIA workstation targeting AI workloads (CUDA/DGX stack). https://www.nvidia.com/en-us/products/workstations/dgx-spark/
- AORUS RTX 5090 AI BOX — external GPU enclosure marketed as an AI box with RTX 5090-class cards. https://www.gigabyte.com/Graphics-Card/GV-N5090IXEB-32GD | Blog: https://global.aorus.com/blog-detail.php?i=1349
- AORUS RTX 5060 Ti AI BOX — similar external GPU concept at a lower tier. https://www.gigabyte.com/Graphics-Card/GV-N506TIXEB-16GD | Blog: https://global.aorus.com/blog-detail.php?i=1345
- Yeren Local AI Box (RNT, Germany) — on-prem local AI workstation. https://rnt.de/en/solutions/yeren-local-ai-special-offer/
- Seeed reComputer AI Industrial R2145-12 — rugged/edge AI box for industrial deployments. https://www.seeedstudio.com/reComputer-AI-Industrial-R2145-12-p-6486.html

**Relevance to Goni:** competitive yardsticks for "turn-key local AI box" expectations (enclosure quality, thermals/acoustics, bundled software).

### 7.2 Mini PCs / nodes (Mac mini-class)
- Apple Mac mini (M4 / M4 Pro) — compact Apple Silicon node often used for local AI. https://www.apple.com/mac-mini/
- GEEKOM A9 Max AI Mini PC (Ryzen AI 9 HX 370) — Ryzen AI mini PC. https://www.geekompc.com/geekom-a9-max-mini-pc/
- GEEKOM AMD Mini PC lineup — broader Ryzen-based mini PC range. https://www.geekompc.com/amd-mini-pc/
- MSI Cubi Z AI 8M — compact Ryzen mini PC marketed for AI-enhanced workloads. Review: https://www.itpro.com/hardware/the-msi-cubi-z-ai-8m-is-an-affordable-mini-pc-with-powerful-amd-radeon-graphics-but-they-really-shouldnt-have-bothered-with-the-speakers
- "Best mini PCs 2025" roundups (TechRadar etc.) — price/perf baselines across Mac mini / Geekom / Minisforum / GMKtec peers. https://www.techradar.com/best/mini-pcs

**Relevance to Goni:** baseline references for silent/compact nodes if running Goni OS on third-party hardware before shipping a custom box.

### 7.3 Decentralized GPU / AI compute networks (Cocoon-style)
- Cocoon — TON-based confidential compute network for AI/GPU mining. https://cocoon.org/ | Intro: https://medium.com/@tort_mario/cocoon-a-new-era-of-ai-compute-mining-by-pavel-durov-4d3595ba5c1d
- Akash Network — decentralized cloud marketplace with on-demand GPUs. https://akash.network/ | GPUs: https://akash.network/gpus-on-demand/
- Render Network — decentralized GPU rendering expanding into AI/general compute. https://rendernetwork.com/ | GPU onboarding: https://renderfoundation.com/gpu | AI expansion: https://rendernetwork.medium.com/why-rnp-019-matters-a-pivotal-expansion-for-render-network-into-general-and-ai-compute-cbabe7333e45
- io.net — decentralized GPU network for AI/ML workloads. https://io.net/ | Overview: https://www.coingecko.com/learn/what-is-io-net-io-token
- Spheron Network — decentralized GPU/CPU compute for AI/HPC. https://spheron.network/ | Whitepaper: https://www.spheron.network/whitepaper/
- Bittensor — decentralized AI network (TAO) for model training/inference incentives. https://bittensor.com/ | Paradigm: https://bittensor.com/about
- OORT — decentralized cloud for AI data & compute. https://www.oortech.com/ | Links: https://linktr.ee/oortech

**Relevance to Goni:** potential hybrid/offload paths (burst or heavy jobs to DePIN nodes) and a resale path for idle local GPU cycles; see also `docs/remote-llm-architecture.md` for how remote routing is handled in software.

---

## 8. References

1. GMKtec EVO-X2 product page, GMKtec.
   https://de.gmktec.com/en/products/gmktec-evo-x2-amd-ryzen%E2%84%A2-ai-max-395-mini-pc-1

2. GMKtec EVO-X2 Amazon listing.
   https://www.amazon.com/GMKtec-ryzen_ai_mini_pc_evo_x2/dp/B0F53MLYQ6

3. HP Z2 Mini G1a official product page, HP Inc.
   https://www.hp.com/us-en/workstations/z2-mini-a.html

4. HP Z2 Mini G1a (AMD Ryzen AI Max+ PRO 395, 64GB, 2TB) - example EU listing, Bechtle.
   https://shop.bechtle-plm.com/produkt/cad-workstations/hp-z2-mini-g1a-amd-ryzen-ai/

5. Framework Desktop mainboard (AMD Ryzen AI Max 300 series) product page, Framework.
   https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series

6. Framework Desktop overview page.
   https://frame.work/desktop

7. Acer Veriton GN100 AI Mini Workstation product page, Acer.
   https://www.acer.com/us-en/desktops-and-all-in-ones/veriton-workstations/veriton-gn100-ai-mini-workstation

8. Acer press release: "Acer Unveils the Veriton GN100 AI Mini Workstation Built on the NVIDIA GB10 Superchip", 3 Sept 2025.
   https://news.acer.com/acer-unveils-the-veriton-gn100-ai-mini-workstation-built-on-the-nvidia-gb10-superchip

9. Tom's Hardware: "Acer unveils Project Digits supercomputer featuring Nvidia's GB10 superchip with 128GB of LPDDR5x", 2025.
   https://www.tomshardware.com/tech-industry/artificial-intelligence/acer-unveils-project-digits-supercomputer-featuring-nvidias-gb10-superchip-with-128gb-of-lpddr5x

10. TechRadar Pro: "Acer joins Nvidia GB10 superchip fan club with ,999 Veriton AI mini workstation PC...", 2025.
    https://www.techradar.com/pro/acer-joins-nvidia-gb10-superchip-fan-club-with-usd3-999-veriton-ai-mini-workstation-pc-and-yes-you-can-link-two-to-get-even-more-power

11. Framework Desktop PC review, PC Gamer, 2025.
    https://www.pcgamer.com/hardware/gaming-pcs/framework-desktop-pc-review/

12. Framework Desktop review, The Verge, 2025.
    https://www.theverge.com/reviews/749404/framework-desktop-pc-amd-ryzen-ai-max-385-395-plus-review
