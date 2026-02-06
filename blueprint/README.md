# Project Goni

 > This is the technical blueprint and prototype index. It lives in `blueprint/` so the repo root can stay non-technical; links here point to the canonical artifacts in this blueprint folder.

> **Goni** is a private operator appliance: a local-first digital double that refines your online life into briefs, decisions, and actions - quietly, with receipts, under your rules. This repo is the open-source blueprint and prototype lab.

> **Status (prototype lab, not shippable hardware)**
> - Blueprint + prototype lab; APIs/BOM may change.
> - Docker/k8s stacks are for experimentation, not production.
> - Manufactured MVP waits on financing; this repo is the reference design it will follow.

## What Goni is (fast)

**User truth**

- Your private operator: always-on, local-first, loyal to your attention.
- A refinery: raw inputs -> distilled facts -> action cards -> durable memory.

**Technical truth**

- An appliance-grade OS + data spine + governance layer + job framework.
- Local models by default; explicit, logged escalation to external models.

**The three surfaces**

- **Daily Brief**: compressed priorities and risks.
- **Action Cards**: binary decisions with context + rationale + receipts.
- **Vault**: your distilled, searchable memory.

**Core loop**

Observe -> Distill -> Propose/Act -> Attach receipts -> Store memory.

**Why it matters**

- Signal protection, time recovery, governed autonomy, sovereignty.

## Kernel guarantees (spec)

Target guarantees; see goni-prototype-lab:goni-lab/STATUS.md for implementation status.

- Tool side effects are intended to be mediated by the kernel boundary.
- Network egress is intended to go through the egress gate.
- Every mediated action is intended to produce a receipt.
- Budgets are intended to be enforced at mediation boundaries.
- Receipts are intended to be minimal by default.

**Trusted computing base (TCB)**

- Kernel mediation and receipt components.
- Egress gate.

## Agility guardrails

- Keep the front page timeless: [README.md](../README.md), [blueprint/docs/goni-story.md](/blueprint/docs/goni-story.md), and [blueprint/docs/goni-whitepaper.md](/blueprint/docs/goni-whitepaper.md) stay gut-punch + vision only. Put concrete numbers and models in [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md), [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md), or `blueprint/40-implementation/**`.
- See [blueprint/docs/goni-agility-rules.md](/blueprint/docs/goni-agility-rules.md) for the full map of what can be pinned where, and how to propose tighter numbers with evidence.

This space is for:

- hardware engineers  
- software engineers  
- stakeholders (product, ops, early adopters)

.to converge on the **best possible Goni**.

---

## How to read this repo

- **Product/story track (stakeholders, early adopters)**: start with [blueprint/docs/goni-story.md](/blueprint/docs/goni-story.md), then [blueprint/docs/goni-whitepaper.md](/blueprint/docs/goni-whitepaper.md) for the deep architecture narrative, and [blueprint/docs/goni-swot.md](/blueprint/docs/goni-swot.md) for positioning.
- **Hardware track (hardware builders)**: [blueprint/hardware/00-overview.md](/blueprint/hardware/00-overview.md) -> [blueprint/hardware/10-requirements.md](/blueprint/hardware/10-requirements.md) -> [blueprint/hardware/20-architecture-options.md](/blueprint/hardware/20-architecture-options.md) -> [blueprint/hardware/25-hardware-layers-and-supplier-map.md](/blueprint/hardware/25-hardware-layers-and-supplier-map.md), with accepted choices in [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md).
- **Software track (software builders)**: [blueprint/software/00-overview.md](/blueprint/software/00-overview.md) -> [blueprint/software/10-requirements.md](/blueprint/software/10-requirements.md) -> [blueprint/software/20-architecture.md](/blueprint/software/20-architecture.md) -> data spine in [blueprint/software/50-data/00-index.md](/blueprint/software/50-data/00-index.md) (and [blueprint/software/50-data/53-schema-dsl-and-macros.md](/blueprint/software/50-data/53-schema-dsl-and-macros.md) for the Arrow DSL) -> accepted choices in [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md).
- **Data spine <-> kernel**: the planes and TXT axiom are defined in [blueprint/software/50-data/10-axioms-and-planes.md](/blueprint/software/50-data/10-axioms-and-planes.md); enforcement is specified only (see [goni-prototype-lab:goni-lab/STATUS.md](/goni-prototype-lab:goni-lab/STATUS.md)).
- **Runs and deployments (I just want to run something)**: see [blueprint/deploy/compose/README.md](/blueprint/deploy/compose/README.md) and [blueprint/deploy/k8s/README.md](/blueprint/deploy/k8s/README.md) for current status.

---

## Doc Map (where the truth lives)

- Agents: [blueprint/30-specs/agent-definition.md](/blueprint/30-specs/agent-definition.md)
- Agent manifests: [blueprint/30-specs/agent-manifest.md](/blueprint/30-specs/agent-manifest.md)
- Latent state contract: [blueprint/30-specs/latent-state-contract.md](/blueprint/30-specs/latent-state-contract.md)
- Tool capability API + audit: [blueprint/30-specs/tool-capability-api.md](/blueprint/30-specs/tool-capability-api.md)
- Network gate + anonymity: [blueprint/30-specs/network-gate-and-anonymity.md](/blueprint/30-specs/network-gate-and-anonymity.md)
- Scheduler + interrupts: [blueprint/30-specs/scheduler-and-interrupts.md](/blueprint/30-specs/scheduler-and-interrupts.md)
- Receipts: [blueprint/30-specs/receipts.md](/blueprint/30-specs/receipts.md)
- Data plane schemas: [blueprint/software/50-data/51-schemas-mvp.md](/blueprint/software/50-data/51-schemas-mvp.md)

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


### 1.3 What Goni is / isn't (positioning wedge)

**Not**

- NAS + local chatbot glued on top
- LangChain loops run locally
- Self-hosted cloud distro

**Is**

- Cognitive OS kernel with audited syscalls
- Agents as local processes with budgets and policy
- Remote presence as a capability, not an implicit port

Reference pattern: "personal local cloud OS" (Olares) in
`blueprint/docs/reference-products/olares.md`, but Goni stays cognition-first.

## 2. Hard constraints (everyone should agree on these)

These are the guardrails for all discussions in this repo. If you propose changes, start here; they are non-negotiable unless a proposal with better numbers moves them.

### 2.1 Use-case constraints

- **Local-first**: 80-90% of tokens should be served by **local models** on Goni.  
- **Cloud-as-needed**: [LLM Council](/blueprint/docs/llm-council.md) used only when:
  - explicitly requested by user, or
  - orchestrator deems task "high difficulty" or requires long context.
  - Remote path is mediated: Goni OS -> Goni Council -> OpenRouter (multi-model gateway) -> cloud providers. Goni never calls provider APIs directly; see [blueprint/docs/remote-llm-architecture.md](/blueprint/docs/remote-llm-architecture.md) for data path, budgets, and runtime modes.

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

This repo is split into lanes:

### `/00-map/`

Navigation and meta (migration ledger, docs rules, reviewer guides).

### `/10-product/`

Vision, roadmap, next steps, and product-facing research agendas.

### `/20-system/`

System-level summaries (trust posture, performance framing).

### `/30-specs/`

Canonical normative contracts.

### `/decisions/`

ADRs and cross-cutting decision notes.

### `/50-evidence/`

Benchmarks, evals, and reproducibility artifacts.

### `/60-market/`

Competitors, suppliers, and partner research.

### `/hardware/`

Physical layer: enclosure, BOM, thermals, electronics.

- component choices (APU, PSU, SSDs)  
- thermal design & airflow  
- enclosure + mechanical drawings  
- front-panel MCU, LEDs, buttons  
- BOM experiments and cost models  

Start in:

- [blueprint/hardware/00-overview.md](/blueprint/hardware/00-overview.md) - current hardware concept  
- [blueprint/hardware/10-requirements.md](/blueprint/hardware/10-requirements.md) - constraints and goals  
- [blueprint/hardware/20-architecture-options.md](/blueprint/hardware/20-architecture-options.md) - alternative designs

Use [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md) to record accepted decisions.

### `/software/`

Kernel, orchestrator, and infra for the node/mesh:

- base OS and provisioning  
- orchestrator architecture  
- model serving (local LLMs)  
- mesh design and coordination  
- APIs and UI

Start in:

- [blueprint/software/00-overview.md](/blueprint/software/00-overview.md) - current software concept  
- [blueprint/software/10-requirements.md](/blueprint/software/10-requirements.md) - functional & non-functional requirements  
- [blueprint/software/20-architecture.md](/blueprint/software/20-architecture.md) - formal architecture and invariants  
- [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md) - formal design decisions  
- Kernel implementation: specified only at the moment; see [goni-prototype-lab:goni-lab/STATUS.md](/goni-prototype-lab:goni-lab/STATUS.md)

### `/docs/`

Story and narrative architecture:

- [blueprint/docs/goni-whitepaper.md](/blueprint/docs/goni-whitepaper.md) - core technical + architectural spec (planes, Arrow spine, control layer).  
- [blueprint/docs/goni-story.md](/blueprint/docs/goni-story.md) - narrative "Goni story" (magazine-style for tech enthusiasts / early adopters).  
- [blueprint/docs/goni-swot.md](/blueprint/docs/goni-swot.md) - honest SWOT vs Atlantis / OpenDAN / Home Assistant / mini-PC stacks.  
- [blueprint/docs/inspiration.md](/blueprint/docs/inspiration.md) - builders & thinkers we track.  
- [blueprint/docs/related-projects.md](/blueprint/docs/related-projects.md) - prior art / similar systems and how we differ.  
- [blueprint/docs/glossary.md](/blueprint/docs/glossary.md) - shared vocabulary.  
- [blueprint/docs/diagrams/](/blueprint/docs/diagrams/) - architecture diagrams (Draw.io, Figma exports).  
- [blueprint/docs/README.md](/blueprint/docs/README.md) - local docs index linking all of the above.

---

## 6. How to contribute

1. **Read the constraints (Section 2)** - treat them as the current baseline.  
2. For **hardware topics**, open:
   - an issue with label `hardware`, and  
   - reference the relevant file in `/hardware/`.  

3. For **software topics**, label `software` and reference `/software/`.  

4. Major changes should come with:
   - a short proposal (`proposal-*.md`) and  
   - a suggested update to [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md) or [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md).

5. Keep discussion as concrete as possible:  
   - numbers (W, GB, TB, TOPS, latency)  
   - cost estimates ($)  
   - actual vendor links if relevant

We are optimising for a clean, well-argued architecture. Changes that affect constraints/architecture should come with numbers and a short proposal.

If you are thinking about manufacturing/funding, open an issue; the hardware build triggers when the blueprint feels solid and a partner is ready.

---

## Quickstart (local stack)

Prototype/dev stack is specified only; things may change between commits.

Prereqs: Docker and docker compose. See goni-prototype-lab:goni-lab/quickstart.md for the intended workflow.

Services:  
- `llm-local` (vLLM) at `http://localhost:8000/v1`  
- `vecdb` (Qdrant) at `http://localhost:6333`  
- `orchestrator` (goni-http) at `http://localhost:7000`  

Env vars of interest:  
- `LLM_LOCAL_URL` (default: `http://llm-local:8000/v1`)  
- `LLM_MODEL` (default: `mistralai/Mistral-7B-Instruct-v0.3`)  
- `QDRANT_HTTP_URL` (default: `http://vecdb:6333`)  
- `QDRANT_COLLECTION` (default: `default`)  
- `EMBED_DIM` (default: `1024`)

Example call (against llm-local):  
`curl http://localhost:8000/v1/models`

## Quickstart (k8s / k3s)

Prototype/dev overlays are specified only; see [blueprint/deploy/k8s/README.md](/blueprint/deploy/k8s/README.md).

