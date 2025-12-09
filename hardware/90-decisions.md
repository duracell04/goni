# 90 - Hardware Decisions (ADR Log)

> This file is a **decision log** for Goni hardware.
> Each entry records one important, relatively stable decision about the hardware architecture or implementation.

The goal is **not** to document every small tweak, but to capture the **big choices** that shape the product so that:

- engineers can see why something is the way it is,
- new contributors can understand the constraints,
- we avoid repeating the same debates.

Decisions here should always reference the more detailed documents in this folder, for example:

- [10-requirements.md](./10-requirements.md) - hardware requirements,
- [20-architecture-options.md](./20-architecture-options.md) - candidate architectures,
- 50-bom-experiments/ - BOM experiments and cost models.

---

## ADR Template

When adding a new decision, copy this template and fill it out:

`markdown
### ADR-XXX - Short title of the decision

**Status:** Proposed / Accepted / Deprecated
**Date:** YYYY-MM-DD
**Owner:** @github-handle

#### Context

- Briefly describe the background.
- What requirements from 10-requirements.md are relevant?
- Which options from 20-architecture-options.md / BOM experiments were considered?

#### Decision

- State the decision clearly in 2-3 bullet points.
- Specify what we **will do** and, if relevant, what we explicitly **will not do**.

#### Consequences

- What becomes easier because of this decision?
- What becomes harder or impossible?
- Are there follow-up tasks (e.g. update BOM, CAD, firmware assumptions)?

#### References

- Link to discussion issues / PRs.
- Link to relevant documents in this repo (architecture, BOM experiments, etc.).
`

Numbering (ADR-001, ADR-002, ...) can be assigned in increasing order. We do not need perfect global sequencing; uniqueness and clarity matter more than strict order.

---

## ADR-001 - Baseline Architecture: APU-Centric Goni Node

**Status:** Proposed
**Date:** (fill when accepted)
**Owner:** (assign)

#### Context

We need to choose a **baseline hardware architecture** for the Goni MVP node that satisfies:

- small appliance form factor (~6-8 L),
- low acoustic footprint (quiet under typical AI workloads),
- ability to run **medium-to-large LLMs** locally (~30-40B parameters quantised),
- reasonable power consumption (few hundred watts, single household outlet),
- ability to **mesh** multiple nodes into a personal cluster,
- and a path to **future upgrades** (compute board swaps, external heavy nodes).

10-requirements.md defines these constraints.
20-architecture-options.md surveys three main options:

1. APU-centric node (integrated CPU/GPU/NPU + unified LPDDR5X),
2. discrete GPU workstation (desktop CPU + high-end dGPU),
3. external Grace Blackwell node (Acer Veriton GN100) as a heavy add-on.

Initial BOM work in 50-bom-experiments/bom-v1-apu-node.md suggests that an APU-centric node is technically feasible with a **BOM around USD 2.8-3.5k** while satisfying size and power constraints.

#### Decision

- The **baseline Goni MVP node** will be **APU-centric**:

  - one high-end APU with integrated CPU, GPU and NPU,
  - **unified LPDDR5X memory** (no separate VRAM),
  - target capacity **128 GB**.

- The Goni enclosure and power system will be designed for:

  - a **single APU board** in a Mini-ITX-compatible form factor,
  - ~**200-250 W** sustained load with comfortable cooling,
  - one **small PSU** (~500-600 W) and **two NVMe SSDs** (OS + data).

- Discrete GPU workstations and external Grace Blackwell nodes will be treated as:

  - **future extensions** (e.g. "Goni Lab" or "Goni Max"),
  - not part of the MVP node's default hardware.

#### Consequences

**Positive:**

- Simplifies thermal and acoustic design:

  - total power remains in a range manageable in <10 L without server-like noise.
- Enables a compact, homogenous **cluster node** design - all nodes share the same APU profile.
- Unified memory (single pool) simplifies local LLM deployment for 30-40B models and reduces complexity around VRAM vs system RAM.
- BOM experiments show we can hit a **USD ~2.8-3.5k** hardware cost while justifying a **USD 10k** retail price including personalisation and setup.

**Negative / trade-offs:**

- Memory (LPDDR5X) is **soldered** and not user-upgradeable; capacity must be set at manufacturing time.
- No ECC in current consumer LPDDR5X implementations.
- Peak training throughput and extremely large model support (70B+ full precision) will remain better on discrete GPU or GN100-class systems; Goni MVP will **not** be optimised for such workloads.

**Follow-ups:**

- Lock the **target memory capacity** (64 vs 128 GB) and document it explicitly in 10-requirements.md and BOM files.
- Update mechanical design in 30-mechanical/ to assume a single APU mainboard with SFX/ATX PSU and 2x NVMe slots.
- Align software assumptions (e.g. LLM sizes, quantisation strategies) with the available unified memory.

#### References

- [hardware/10-requirements.md](./10-requirements.md) - Hardware requirements.
- [hardware/20-architecture-options.md](./20-architecture-options.md) - APU vs dGPU vs GN100 comparison.
- [hardware/50-bom-experiments/bom-v1-apu-node.md](./50-bom-experiments/bom-v1-apu-node.md) - BOM experiment for APU-centric Goni node.
- External primary references for example APU boards and devices (GMKtec, HP Z2, Framework, Acer GN100) are listed in 20-architecture-options.md and om-v1-apu-node.md.

---

## ADR-002 - IPW as Primary Hardware Metric & Hybrid-Routing Design Constraint

**Status:** Proposed
**Date:** (fill when accepted)
**Owner:** (assign)

#### Context

- Local-first claim needs a **per-watt intelligence** yardstick across APU, dGPU, GN100-class nodes.
- Saad-Falcon et al. (2025) define **Intelligence per Watt (IPW)** and provide a public profiling harness over real LLM traffic (1M single-turn queries) covering 20+ local models and 8 accelerators (M-series, AMD, NVIDIA). Their results show:
  - ~5.3x IPW uplift from 2023→2025 (model + accelerator improvements combined).
  - Local↔cloud gap of ~1.4–1.5x IPW, implying edge nodes still have headroom.
  - Oracle hybrid routing could cut energy ~80.4% and cost ~73.8% vs cloud-only baselines, with most savings retained even with imperfect routers.
- We need a **standard benchmark/harness** to compare candidate Goni boxes and to size hybrid routing policies.

#### Decision

- **Adopt IPW as the primary hardware efficiency metric** for evaluating any “model × hardware” combo we consider for Goni.
- **Treat hybrid routing efficiency as a design constraint**, not an afterthought: every hardware evaluation must report expected IPW and hybrid-routing savings relative to a cloud-only baseline.
- **Integrate the authors’ IPW profiling harness** (or equivalent reproduction) into the prototype track to run on:
  - baseline APU nodes,
  - optional dGPU add-ons,
  - GN100-class lab nodes.

#### Consequences

- Hardware choices (boards, accelerators, cooling) must report IPW under representative loads, not just TOPS/TFLOPS.
- Routing policy experiments need to include “energy/cost vs accuracy” plots so we can justify local-first defaults and when to burst to cloud.
- BOM and thermal design discussions gain a common yardstick; we can prune options that look good on paper but have weak IPW.
- Adds work: the prototype harness must be maintained and run across candidate configs before promoting them into decisions.

#### References

- Saad-Falcon, J. et al. (2025). *Intelligence per Watt: Measuring Intelligence Efficiency of Local AI*. arXiv:2511.07885.
- IPW profiling harness (public): see the paper’s accompanying repository.

---

*(Add further ADRs below as decisions are made, e.g. enclosure dimensions, PSU choice, networking baseline, etc.)*
