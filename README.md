# Goni MVP

> **Goni** is a small, local-first AI node - a matte-black box in your home or office - that runs your personal AI assistant on your own hardware, can mesh with other nodes, and optionally talk to the cloud when it truly helps.

This repo is **not** a marketing site. It's a working space for:

- hardware engineers  
- software engineers  
- stakeholders (product, ops, early adopters)

.to converge on the **best possible Goni MVP**.

---

## 1. What Goni MVP is (and is not)

### 1.1 Product idea in one paragraph

Goni is a **local AI super-node** for individuals and small teams:

- Runs **30-40B parameter models** locally (quantised) for chat, coding, and RAG.  
- Trains **adapters / LoRA** on your data (no huge full-model fine-tunes on-device).  
- Works as a **standalone box** _and_ can **mesh with other Goni nodes** to form a personal cluster.  
- Can attach a **Grace Blackwell GN100-class mini-DGX** later for "Goni Max / Enterprise".

From the user's perspective:  
> "I buy a box once for ~$10k, then I pay $750/month and my AI is faster, more private, and more predictable than whatever cloud dashboard I was using before."

### 1.2 Non-goals (for MVP)

To keep us aligned:

- No attempt to match full **H100/B200 training rigs** locally.  
- No exotic multi-GPU monster towers for v1.  
- No gamer aesthetics (RGB, glass, etc.).  
- No "just a rebadged OEM box" - Goni has its own industrial design and OS image.

---

## 2. Hard constraints (everyone should agree on these)

These are the guardrails for all discussions in this repo.

### 2.1 Use-case constraints

- **Local-first**: 80-90% of tokens should be served by **local models** on Goni.  
- **Cloud-as-needed**: GPT-4 / Claude / etc. used only when:
  - explicitly requested by user, or
  - orchestrator deems task "high difficulty" or requires long context.

- **Workload focus**:
  - Inference + RAG for 30-40B quantised models  
  - Adapters / LoRA / "personalisation" training only  
  - Heavy full-model fine-tune ? kicked to cloud or GN100-class node

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

### 2.3 UX & ownership constraints

- Local-first and **offline-capable**: box must still be useful with no internet.  
- **Full ownership**:
  - User owns the hardware, the base OS image, and their models/checkpoints.  
  - Telemetry is opt-in, not built-in.

- **Friendly setup**:
  - First boot: Goni appears as `GONI-SETUP` Wi-Fi or at a local URL (e.g. `https://goni.local`).  
  - Wizard: admin password, network config, optional WireGuard key, optional disk encryption.

---

## 3. High-level architecture (10k ft view)

### 3.1 Single node

On a single Goni box we assume:

- **OS**: Ubuntu Server LTS (or similar), encrypted disk optional.  
- **Runtime**: containers (Docker / containerd).  
- **Core services**:
  - `llm-local`: vLLM/TGI bound to the APU (iGPU + CPU).  
  - `vecdb`: Qdrant/Milvus for embeddings and RAG.  
  - `orchestrator`: decides when to use local models vs cloud models; manages tools and workflows.  
  - `gateway`: HTTPS API (OpenAI-compatible) + web dashboard.  
  - `wg-mesh`: WireGuard for secure remote access and node-to-node mesh.

### 3.2 Mesh / cluster

Every Goni node is a **cluster node**:

- 1st node ? control plane (e.g. k3s server).  
- Additional nodes ? join as workers via "join token / URL".

The orchestrator sees accelerators abstractly:

- `apu:0` ? local Ryzen AI APU (iGPU+CPU).  
- `npu:0` ? local NPU (XDNA 2).  
- `gn100:0` ? (future) Grace Blackwell GN100 node in the same mesh.

Tasks:

- **Interactive** ? run on the node closest to the user.  
- **Batch** (embeddings, long research, nightly updates) ? can be offloaded to other nodes or GN100.

---

## 4. Future-proofing assumptions

We design Goni MVP with the following in mind:

- **APU roadmap**: future AMD/Intel APUs with more NPU TOPS and better FP8/FP4 ? we should be able to swap in a new mainboard without changing the Goni case, PSU, or overall architecture.

- **NPU evolution**: NPUs become real inference backends for smaller models (ASR, vision, routing). The orchestrator must treat NPUs as first-class targets, not afterthoughts.

- **GN100 / Blackwell-class nodes**:
  - GN100 today: 128 GB unified memory, 1 PFLOP FP4, 0.5 L form factor, $3.9k+.
  - Future GBxx mini-DGX successors can be added as **"Goni Max nodes"** in the mesh, without redesigning the base Goni.

---

## 5. Repo layout

This repo is split into two main workspaces:

### `/hardware/`

For everything physical:

- component choices (APU, PSU, SSDs)  
- thermal design & airflow  
- enclosure + mechanical drawings  
- front-panel MCU, LEDs, buttons  
- BOM experiments and cost models  

Start in:

- `hardware/00-overview.md` - current hardware concept  
- `hardware/10-requirements.md` - constraints and goals  
- `hardware/20-architecture-options.md` - alternative designs

Use `hardware/90-decisions.md` to record accepted decisions.

### `/software/`

For everything software:

- base OS and provisioning  
- orchestrator architecture  
- model serving (local LLMs)  
- mesh design and coordination  
- APIs and UI

Start in:

- `software/00-overview.md` - current software concept  
- `software/10-requirements.md` - functional & non-functional requirements  
- `software/20-architecture.md` - formal architecture and invariants  
- `software/90-decisions.md` - formal design decisions  
- `software/kernel/` - Rust workspace for the kernel (traits, HTTP server, vLLM client, Qdrant-backed RAG, scheduler/router stubs)

### `/docs/`

Shared documentation:

- `goni-whitepaper.md` - longer-form explanation for non-engineers.  
- `/docs/diagrams/` - architecture diagrams (Draw.io, Figma exports).  
- `glossary.md` - keep our vocabulary aligned.

---

## 6. How to contribute

1. **Read the constraints (Section 2)** - treat them as the current baseline.  
2. For **hardware topics**, open:
   - an issue with label `hardware`, and  
   - reference the relevant file in `/hardware/`.  

3. For **software topics**, label `software` and reference `/software/`.  

4. Major changes should come with:
   - a short proposal (`proposal-*.md`) and  
   - a suggested update to `hardware/90-decisions.md` or `software/90-decisions.md`.

5. Keep discussion as concrete as possible:  
   - numbers (W, GB, TB, TOPS, latency)  
   - cost estimates ($)  
   - actual vendor links if relevant

We are aiming at a **real, buildable MVP**, not a sci-fi spec - but we do want to stay aligned with where hardware and AI tooling will be in **2026-2027**.

---
