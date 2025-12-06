# 25 - Hardware Layers and Supplier Map (2025-2026)

This note captures the **mental model for Goni hardware** and the **suppliers to track** for each layer. Use it as a **one-pager** when making kernel/backend decisions, planning hardware tiers, or talking to partners.

---

## 1. Layered Mental Model

Everything maps to four layers:

- **Silicon / Accelerators** — GPUs, NPUs, ASICs, photonic processors; what `LlmEngine` backends target.
- **Systems / Boxes** — workstations, servers, mini PCs; what sits on a desk or in a rack.
- **Edge / Always-On** — ultra-low-power inference for router/intent models that keep the big GPU asleep.
- **Workloads / Models** — what drives specs (FLUX, 70B LLMs, adapters, multimodal).

`LlmEngine` should stay **backend-pluggable**, with implementations such as: `Cpu`, `Cuda`, `Rocm`, `Gaudi`, `Tenstorrent`, `Photonic`, and an **edge-class** path (e.g. Hailo) for “wake-on-GPU” routing.

---

## 2. Goni Tiers (v1 / Pro / Max)

- **v1 – APU Node / Dev Box**  
  Goal: silent, cheap, hackable; orchestrates Pro/Max.  
  Compute: AMD Ryzen AI / high-end APU (unified memory), optional Hailo module or built-in NPU.  
  Boxes: Framework board/laptop, mini PCs (Minisforum/Beelink), DIY ITX with 128 GB LPDDR/DDR5.  
  Suppliers to lean on: **AMD (APUs), Framework/SFF vendors, Hailo**.

- **Pro – Single Big GPU Exocortex**  
  Goal: real exocortex; runs FLUX + 70B LLMs comfortably.  
  Compute: 1× NVIDIA RTX 4090/5090 or RTX 6000-class (24–32 GB) **or** 1× Intel Gaudi (96+ GB HBM) if hedging against CUDA lock-in.  
  Boxes: AIME / BIZON / aiserver.eu workstation, TUXEDO/System76 tower, or big OEM workstation (Dell/Lenovo/HPE).  
  Suppliers to lean on: **NVIDIA (default), Intel Gaudi (second vendor), AMD MI300X (if embracing ROCm)**.

- **Max – Cluster / Experimental / Photonic**  
  Goal: small “AI lab in a rack”; multi-GPU or mixed accelerators, plus future photonics.  
  Compute: NVIDIA HGX / multi-GPU with NVLink; Intel Gaudi cluster (Ethernet/RoCE); Tenstorrent Wormhole; “Tier-0” photonic cards/servers (Arago, Q.ANT; later Lightmatter/Scintil fabric).  
  Boxes/fabric: Supermicro GPU servers; Lambda/AIME/aiserver.eu integrators; photonic IO/fabric from Scintil/Lightmatter.  
  Suppliers to track: **NVIDIA + Supermicro (baseline), Intel Gaudi + Supermicro (Alt-Max), Arago / Q.ANT / Lightmatter / Scintil (Max-plus)**.

---

## 3. Silicon / Accelerator Suppliers

### 3.1 Mainstream GPUs & NPUs

- **NVIDIA** — GeForce 40/50, RTX/Quadro, H100/B200/GB200, NVLink. Pros: de-facto standard, CUDA/tooling, great for FLUX + 70B. Cons: expensive, lock-in, NVLink mostly DC. Best: **Pro, Max**.
- **AMD** — Ryzen AI APUs, Ryzen/Threadripper/EPYC, MI300X. Pros: v1 APUs, 192 GB HBM on MI300X, more open ROCm. Cons: smaller ecosystem. Best: **v1, Pro, some Max**.
- **Intel (Gaudi)** — Gaudi2/3 with 96+ GB HBM, Ethernet/RoCE fabric. Pros: strong perf/€; HF integration; Ethernet scaling. Cons: ecosystem maturity below CUDA. Best: **Pro, Max**.

### 3.2 “Alternative” AI Accelerators

- **Tenstorrent (Wormhole)** — RISC-V/Tensix PCIe cards up to 24 GB GDDR6. Role: open-ish backend (`LlmEngine::tenstorrent`) for workstations (“TT-QuietBox / LoudBox”). Early but developer-friendly.
- **Groq (LPU)** — Ultra-low-latency inference; ideal router/agent layer. Inference-only; mostly cloud today but conceptually a latency backend.
- **Cerebras (WSE-3)** — Wafer-scale engine; huge models without sharding. Fits **Max lab**; 20 kW+ so DC/colo context only.
- **SambaNova (RDU / DataScale)** — Integrated HW + LLMs (Samba-1). Good if you buy full stack; less modular.

### 3.3 Photonic / Optical Compute (Tier-0 / Future)

- **Arago** — Photonic AI processors + CARLOTA software (PyTorch device). Energy-efficient optical matmuls; future `LlmEngine::photonic`. Early but funded (2025).
- **Lightmatter** — Envise compute + Passage interconnect + photonic fabric/IO. Targets DC bandwidth/power bottlenecks; fits Goni Max fabric roadmap.
- **Q.ANT** — Photonic NPU servers; NPU 2.0 claims up to 30× energy efficiency, 50× perf vs CMOS. Practical PCIe/server photonic accelerator.
- **Scintil Photonics** — LEAF Light DWDM light engine for co-packaged optics; single-chip light engine for GPU clusters. Photonic IO for future racks; NVIDIA investor.

### 3.4 Edge Inference

- **Hailo (8/10 M.2/mini-PCIe)** — Run lightweight router/intent/VAD models 24/7; wake big GPU only when needed. Perfect for “Goni Router” board or inside a mini PC.

---

## 4. Systems / Box Suppliers

- **Big OEMs / Server Builders** — Supermicro (primary Goni Max chassis, 1–8 GPU, air/liquid, HGX B200); Dell / Lenovo / HPE (workstations + 1–4 GPU servers; enterprise-friendly thermals/warranties).
- **AI-Focused Integrators (EU-friendly)** — Lambda (GPU workstations/servers), AIME (custom HPC GPU workstations/servers, EU), BIZON (quiet liquid-cooled RTX workstations), aiserver.eu (H200/DGX-focused), AI-Cube/regional SFF vendors. Good “Goni Pro/Max” OEM partners: you flash Goni OS.
- **Linux-First / “Spiritual Siblings”** — TinyCorp TinyBox (multi-RTX open boxes), TUXEDO Computers (EU Linux-first desktops), System76 (Thelio line), ASN+ (bespoke HW/firmware), Olares (AI appliance UX benchmark).
- **APU / Mini-Node Vendors (v1 focus)** — Framework (modular boards/laptops, Ryzen AI configs), Minisforum/Beelink etc. (SFF APUs, 64–96 GB RAM), DIY ITX/DTX builds (APU + 128 GB RAM).

---

## 5. Workloads / Model Suppliers (Spec Drivers)

- **Black Forest Labs (FLUX.1 / FLUX.2)** — Sets baseline: 24–32 GB VRAM to run high-res FLUX + LLM concurrency.
- **Hugging Face** — Model hub + Gaudi integration; main testbed to validate new backends (Gaudi, Tenstorrent, Q.ANT, etc.).
- **Stability AI / Mistral / others** — SD3, Mixtral/Mistral; additional stress tests on VRAM, bandwidth, token throughput.
- **SambaNova** — Samba-1 LLMs + hardware; good perf baseline even if not adopted.

---

## 6. How to Use This Map

- **Architecture** — Keep `LlmEngine` explicitly backend-pluggable (`Cpu`, `Cuda`, `Rocm`, `Gaudi`, `Tenstorrent`, `Photonic`, edge-class). Treat suppliers as implementations, not special cases.
- **Hardware sequencing (2025–2026 experiments)** — Start with **v1 APU mini-node** (cheap, quiet). Move to **Pro: single RTX 4090/5090 box** from EU integrator (AIME/BIZON). Then explore **Gaudi or Tenstorrent** as second-vendor backends.
- **Partner conversations** — Use shorthand buckets:
  - **Tier-1 GPU partners**: NVIDIA, AMD, Intel.
  - **Tier-1 EU integrators**: AIME, BIZON, aiserver.eu, Supermicro resellers.
  - **Tier-0 future accelerators**: Arago, Q.ANT, Lightmatter, Scintil.
- **Edge story** — Put router/QoS scheduler on **Hailo or built-in NPU**, so the big GPU sleeps until needed.

This file should stay short and opinionated; update it when a supplier becomes real, a backend lands in the kernel, or a tier definition shifts.
