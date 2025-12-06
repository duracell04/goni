# Project Goni

> **Goni** is a blueprint and prototype stack for a compact, local-first AI node you could run in your home or office; it meshes with other nodes and can talk to the cloud when it helps. This is not a boxed product yet.

This repo = open-source blueprint + prototype lab.

> **Status (prototype lab, not shippable hardware)**
> - Blueprint + prototype lab; APIs/BOM may change.
> - Docker/k8s stacks are for experimentation, not production.
> - Manufactured MVP waits on financing; this repo is the reference design it will follow.

This space is for:

- hardware engineers  
- software engineers  
- stakeholders (product, ops, early adopters)

.to converge on the **best possible Goni**.

---

## How to read this repo

- **Product/story track (stakeholders, early adopters)**: start with `docs/goni-story.md`, then `docs/goni-whitepaper.md` for the deep architecture narrative, and `docs/goni-swot.md` for positioning.
- **Hardware track (hardware builders)**: `hardware/00-overview.md` -> `hardware/10-requirements.md` -> `hardware/20-architecture-options.md` -> `hardware/25-hardware-layers-and-supplier-map.md`, with accepted choices in `hardware/90-decisions.md`.
- **Software track (software builders)**: `software/00-overview.md` -> `software/10-requirements.md` -> `software/20-architecture.md` -> data spine in `software/50-data/00-index.md` (and `53-schema-dsl-and-macros.md` for the Arrow DSL) -> accepted choices in `software/90-decisions.md`.
- **Data spine <-> kernel**: the planes and TXT axiom are defined in `software/50-data/10-axioms-and-planes.md` and enforced in code at `software/kernel/goni-schema/src/lib.rs`.
- **Runs and deployments (I just want to run something)**: for a quick local stack, see `software/docker-compose.yml`; for cluster overlays, see `software/k8s/`.

---

## 1. What Goni MVP is (and is not)

### 1.1 Product idea in one paragraph

This is the **target experience** we are converging toward (not what you get by cloning the repo today).

Goni is a **local AI super-node** for individuals and small teams:

- Runs **30-40B parameter models** locally (quantised) for chat, coding, and RAG.  
- Trains **adapters / LoRA** on your data (no huge full-model fine-tunes on-device).  
- Works as a **standalone box** _and_ can **mesh with other Goni nodes** to form a personal cluster.  
- Can attach a **Grace Blackwell GN100-class mini-DGX** later for "Goni Max / Enterprise".

Target user experience once the physical MVP exists:  
> "I buy a high-tech box once for a fixed amount, then I pay a monthly subscription. My local AI (plus the AI council it meets when it goes on the internet) is faster, more private, and more predictable than whatever cloud dashboard I was using before. This gives me a huge boost in productivity and makes chore-like interactions with technology redundant."

### 1.2 Non-goals (for MVP)

To keep us aligned:

- No attempt to match full **H100/B200 training rigs** locally.  
- No exotic multi-GPU monster towers for v1.  
- No gamer aesthetics (RGB, glass, etc.).  
- No "just a rebadged OEM box" - Goni has its own industrial design and OS image.  
- This repo is not a consumer product; it is the reference design the product will follow once funded.

---

## 2. Hard constraints (everyone should agree on these)

These are the guardrails for all discussions in this repo. If you propose changes, start here; they are non-negotiable unless a proposal with better numbers moves them.

### 2.1 Use-case constraints

- **Local-first**: 80-90% of tokens should be served by **local models** on Goni.  
- **Cloud-as-needed**: GPT-4 / Claude / etc. used only when:
  - explicitly requested by user, or
  - orchestrator deems task "high difficulty" or requires long context.

- **Workload focus** (human-speed chat and assistant latency, not API-scale throughput):
  - Inference + RAG for 30-40B quantised models  
  - Adapters / LoRA / "personalisation" training only  
  - Heavy full-model fine-tune -> kicked to cloud or GN100-class node

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

## 3. High-level architecture

### 3.1 Single node

User -> gateway -> orchestrator -> {llm-local, vecdb, tools}, all running in containers on Ubuntu Server.

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

- 1st node -> control plane (e.g. k3s server).  
- Additional nodes -> join as workers via "join token / URL".

The orchestrator sees accelerators abstractly:

- `apu:0` -> local Ryzen AI APU (iGPU+CPU).  
- `npu:0` -> local NPU (XDNA 2).  
- `gn100:0` -> (future) Grace Blackwell GN100 node in the same mesh.

Tasks:

- **Interactive** -> run on the node closest to the user.  
- **Batch** (embeddings, long research, nightly updates) -> can be offloaded to other nodes or GN100.

The prototype code models this routing, even if not fully implemented yet. GN100-class nodes are treated as just another accelerator in the mesh.

---

## 4. Future-proofing assumptions

We design Goni MVP with the following in mind:

- **APU roadmap**: future AMD/Intel APUs with more NPU TOPS and better FP8/FP4 -> we should be able to swap in a new mainboard without changing the Goni case, PSU, or overall architecture.

- **NPU evolution**: NPUs become real inference backends for smaller models (ASR, vision, routing). The orchestrator must treat NPUs as first-class targets, not afterthoughts.

- **GN100 / Blackwell-class nodes**:
  - GN100 today: 128 GB unified memory, 1 PFLOP FP4, 0.5 L form factor, $3.9k+.
  - Future GBxx mini-DGX successors can be added as **"Goni Max nodes"** in the mesh, without redesigning the base Goni.

The blueprint and prototypes remain open-source; the funded MVP will build from this public spec, not a private fork.

---

## 5. Repo layout

This repo is split into two main workspaces:

### `/hardware/`

Physical layer: enclosure, BOM, thermals, electronics.

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

Kernel, orchestrator, and infra for the node/mesh:

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

Story and narrative architecture:

- `/docs/goni-whitepaper.md` - core technical + architectural spec (planes, Arrow spine, control layer).  
- `/docs/goni-story.md` - narrative "Goni story" (magazine-style for tech enthusiasts / early adopters).  
- `/docs/goni-swot.md` - honest SWOT vs Atlantis / OpenDAN / Home Assistant / mini-PC stacks.  
- `/docs/inspiration.md` - builders & thinkers we track.  
- `/docs/related-projects.md` - prior art / similar systems and how we differ.  
- `/docs/glossary.md` - shared vocabulary.  
- `/docs/diagrams/` - architecture diagrams (Draw.io, Figma exports).  
- `/docs/README.md` - local docs index linking all of the above.

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

We are optimising for a clean, well-argued architecture. Changes that affect constraints/architecture should come with numbers and a short proposal.

If you are thinking about manufacturing/funding, open an issue; the hardware build triggers when the blueprint feels solid and a partner is ready.

---

## Quickstart (local stack)

Prototype/dev stack only; things may change between commits.

Prereqs: Docker, docker-compose. From repo root:

1. Build kernel image (or let compose build):  
   `docker build -t ghcr.io/duracell04/goni-http:latest software/kernel`

2. Run stack:  
   `docker-compose -f software/docker-compose.yml up`

Services:  
- `llm-local` (vLLM) at `http://localhost:8000/v1`  
- `vecdb` (Qdrant) at `http://localhost:6333`  
- `orchestrator` (goni-http) at `http://localhost:7000`  
- `gateway` mapped from port 3000 -> 443 (adjust in compose as needed)

Env vars of interest:  
- `LLM_LOCAL_URL` (default: `http://llm-local:8000/v1`)  
- `LLM_MODEL` (default: `mistralai/Mistral-7B-Instruct-v0.3`)  
- `QDRANT_HTTP_URL` (default: `http://vecdb:6333`)  
- `QDRANT_COLLECTION` (default: `default`)  
- `EMBED_DIM` (default: `1024`)

Example call (against llm-local):  
`curl http://localhost:8000/v1/models`

## Quickstart (k8s / k3s)

Prototype/dev overlays for simulation and experimentation.

Prereqs: kubectl + kustomize, k3s/cluster with storage class.

1. Build/push images (adjust registry):  
   `docker build -t ghcr.io/duracell04/goni-http:latest software/kernel`  
   `docker push ghcr.io/duracell04/goni-http:latest`

2. Deploy single-node overlay:  
   `kubectl apply -k software/k8s/overlays/single-node`

Services:  
- `llm-local.goni.svc:8000`  
- `vecdb.goni.svc:6333/6334`  
- `orchestrator.goni.svc:7000`  
- `gateway.goni.svc:80` (ingress at `goni.local` if using provided ingress)

PVC: `goni-models-pvc` (50Gi) created by base manifests. Adjust storage class if needed.
