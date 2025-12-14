# 20 - Candidate Hardware Architectures for Goni (MVP)

Last refreshed: **2025-12-14**

This document surveys candidate **hardware architectures** for the Goni MVP node, and converges on a concrete **MVP reference SKU** so that mechanical/electrical/software can be validated end-to-end.

Inputs:

- requirements: [`10-requirements.md`](./10-requirements.md)
- supplier landscape: [`25-hardware-layers-and-supplier-map.md`](./25-hardware-layers-and-supplier-map.md)
- decisions: [`90-decisions.md`](./90-decisions.md)

---

## 1. What the MVP must deliver (recap)

The MVP node must:

- Fit a **small, quiet appliance** envelope (target ~7 L, allowed 6–8 L).
- Run **two local OSS models in parallel** (typical: 8–14B 4–5-bit quant) plus RAG indexing.
- Sustain a **domestic power + acoustics** profile (few hundred watts max; quiet under interactive use).
- Support **cluster/mesh** operation over Ethernet (2–4 nodes without special switching).
- Be upgradeable by **swapping the compute module** without redesigning the whole enclosure.

---

## 2. Architecture A (MVP): APU-centric node with unified memory

### 2.1 Why this is the leading MVP architecture

An APU-centric node (CPU + iGPU + NPU + unified LPDDR5X) is currently the best match for:

- compact enclosure and quiet cooling,
- “enough” GPU acceleration for quantised inference,
- large unified memory for model + KV cache + embeddings,
- low integration complexity (single compute board + standard PSU + NVMe).

The key requirement for this architecture to feel “real” is **128 GB unified memory**.

### 2.2 Concrete 2025/2026 SKUs that exist today

These are the reference devices/boards that make the APU-centric concept concrete:

#### A1) Framework Desktop Mainboard (primary MVP compute module)

- Product: Framework Desktop Mainboard (AMD Ryzen AI Max 300 series)
- Config: **Ryzen AI Max+ 395 – 128 GB**
- Store price snapshot: **$1,699 (excl. tax)** on Framework Marketplace (pre-order batches).  
  Source: Framework product page.

Why it matters for Goni:

- sold as a **standalone mainboard**, so we can build our own enclosure/PSU/front panel,
- Mini-ITX-style mounting + standard ATX PSU compatibility makes mechanical integration realistic,
- provides a clean “board swap” path for future APU generations.

Links:
- https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series?v=FRAFMK0002

#### A2) HP Z2 Mini G1a (off-the-shelf fallback box)

- Product: **HP Z2 Mini G1a Workstation**
- Example CH config: Ryzen AI Max+ PRO 395, **128 GB**, 1 TB SSD, Radeon 8060S
- Price snapshot (HP CH store): **CHF 3'299 incl. VAT**  
  Price snapshot (Toppreise): ~**CHF 2'883** (varies by reseller and config)

Why it matters for Goni:

- immediately usable as a dev/reference machine,
- proves the “APU + 128 GB unified memory in a tiny box” thesis,
- but is less modular than a board-based design.

Links:
- https://www.hp.com/ch-en/shop/products/desktops/hp-z2-mini-g1a-workstation-desktop-pc-a40q4et-uuz
- https://www.toppreise.ch/preisvergleich/Server-Workstations/HP-Z2-Mini-G1a-Workstation-AMD-Ryzen-AI-Max-PRO-395-A40Q4ET-p816469

#### A3) GMKtec EVO-X2 (mini-PC yardstick)

- Product: **GMKtec EVO-X2** (Ryzen AI Max+ 395)
- Example CH listing (Digitec): **CHF 2'299** (128 GB / 2 TB, delivery depends on batch)

Why it matters for Goni:

- useful as an availability/price yardstick for “APU box” market reality,
- less useful as a Goni hardware base because it’s a closed OEM mini-PC.

Links:
- https://de.gmktec.com/en/products/gmktec-evo-x2-amd-ryzen%E2%84%A2-ai-max-395-mini-pc-1
- https://www.digitec.ch/en/s1/product/gmktec-evo-x2-amd-ryzen-ai-max-395-128gb-ram-2tb-ssd-pc-61575547

---

## 3. Architecture B (Pro / Lab): discrete GPU workstation (x86 + NVIDIA dGPU)

This architecture is ideal for “Pro” tiers and lab workloads, but it conflicts with the MVP envelope:

- louder and larger in practice,
- much higher power budget,
- bigger thermal design burden.

Still, it is the cleanest path to “CUDA-first” toolchains and vLLM throughput.

### 3.1 Current reference GPU options

- **RTX 4090 (24 GB)**: still widely used and available; good perf/CHF in late 2025.
- **RTX 5090 (32 GB GDDR7)**: higher VRAM and Blackwell features; availability depends on region and supply.  
  Specs reference: NVIDIA GeForce RTX 5090 page.

Links:
- https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/

---

## 4. Architecture C (Max add-on): external Grace/Blackwell mini-node

Treat this as a future “add-on accelerator node” for very heavy workloads. It is not required for MVP.
(If we standardise an offload API, Goni can route some jobs to it over Ethernet.)

Reference:
- Acer Veriton GN100 (GB10 / “Project Digits”-class mini workstation)

---

## 5. MVP convergence (what we choose for the next build cycle)

To unblock mechanical/electrical/software validation, we converge on:

1. **MVP compute module (reference):** Framework Desktop Mainboard, Ryzen AI Max+ 395, **128 GB** unified LPDDR5X.  
2. **MVP fallback dev box:** HP Z2 Mini G1a (same memory class, off-the-shelf).  
3. **MVP enclosure assumptions:** ~7 L box, internal SFX PSU, 2× NVMe, front status bar.

This is formalised in [`90-decisions.md`](./90-decisions.md).

---

## 6. Resolved questions

These were “open questions” in earlier drafts; they are now resolved for MVP:

1. **Memory capacity:** MVP requires **128 GB** unified memory. 64 GB is “dev-only” and not performance-representative for the product story.
2. **Mechanical envelope:** target **~7 L**, allowing 6–8 L to accommodate real cooling/PSU/fans.
3. **Networking:** prefer **5 GbE** on the compute module; 2.5 GbE is acceptable only as a fallback.
4. **Board choice:** design around a **Mini-ITX-style mounting + ATX PSU** assumption so we are not hard-locked to one vendor, but use Framework as the first concrete reference.
5. **External heavy node story:** GN100-class nodes are explicitly **out-of-scope for MVP**; revisit once the offload API and mesh are stable.

---

## 7. 2026 watchlist (things that might change the decision)

This section exists so we can refresh quickly when 2026 silicon lands.

### 7.1 Intel Panther Lake (Core Ultra 300, 18A)

Intel has announced Panther Lake as the next-generation client platform and has signalled a major push for “AI PC” capabilities. This is a watch item, not a current MVP candidate, because:

- memory capacity and board availability in “unified 128 GB class” small desktops is unknown,
- we need stable local inference tooling and drivers for the chosen stack.

Reference (official Intel newsroom):
- https://newsroom.intel.com/client-computing/intel-unveils-panther-lake-architecture-first-ai-pc-platform-built-on-18a

### 7.2 AMD next steps after current Ryzen AI Max class

Track successor generations (often discussed as “Strix Halo successor / Medusa” in the community). Treat anything beyond the currently shipping boards as non-binding until we have:

- a real board we can buy,
- published memory configuration options,
- and driver/tooling validation for LLM inference.

---

## 8. What to update next (if you touch this file)

When you refresh this document:

- update the “last refreshed” date,
- re-check availability + pricing for the concrete SKUs,
- ensure the MVP convergence still matches `90-decisions.md`,
- and note any software/backend constraints that would block a given hardware choice.
