# 90 - Hardware Decisions (ADR Log)
DOC-ID: ADR-INDEX-HW

Last refreshed: **2026-01-03**

This file is the **decision log** for Goni hardware. It captures the big, relatively stable choices so we do not re-litigate the same debates and so mechanical/electrical/software can converge on one buildable MVP.

Links:
- requirements: [`10-requirements.md`](./10-requirements.md)
- architectures: [`20-architecture-options.md`](./20-architecture-options.md)
- BOMs: [`50-bom-experiments/`](./50-bom-experiments/)

---

## ADR-001 - Baseline MVP architecture: APU-centric unified-memory node

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware

### Context

We need an MVP node that is:

- small (6–8 L), quiet, and always-on,
- capable of running two local OSS models in parallel plus RAG,
- power-feasible on a standard outlet,
- and upgradeable by swapping the compute module.

`20-architecture-options.md` compares:

1. APU-centric node (CPU+iGPU+NPU + unified LPDDR5X),
2. discrete GPU workstation (x86 + high-end dGPU),
3. external heavy node (Grace/Blackwell class) as add-on.

### Decision

- The **baseline Goni MVP node is APU-centric**:
  - Ryzen AI Max+ 395 class APU (or equivalent generation),
  - **128 GB unified LPDDR5X** (soldered),
  - Mini-ITX-style mounting + standard PSU compatibility.
- Discrete GPU workstations and external heavy nodes are **Pro/Max tiers**, not MVP baseline.

### Consequences

- Mechanical design can target a realistic appliance envelope (~7 L) with quiet cooling.
- We must be explicit about runtime/tooling support on the chosen APU platform (tracked in ADR-006 / software runtime work).

### References

- [`20-architecture-options.md`](./20-architecture-options.md)
- [`50-bom-experiments/bom-v2-framework-395-128gb.md`](./50-bom-experiments/bom-v2-framework-395-128gb.md)

---

## ADR-002 - MVP memory floor: 128 GB unified memory is required

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware

### Context

The MVP experience target includes:

- two local models in parallel (8–14B quant),
- interactive context lengths (8–16k),
- RAG indexes and embeddings in memory.

64 GB unified memory can run smaller models, but it is not representative of the “real exocortex” story and tends to collapse under concurrency.

### Decision

- MVP reference builds use **128 GB unified memory**.
- 64 GB devices may be used for early dev/testing, but are not the performance reference.

### Consequences

- Compute module choice is constrained to 128 GB configurations (often soldered LPDDR).
- BOM and supply chain focus on 128 GB availability; no “upgrade later” assumption.

### References

- [`10-requirements.md`](./10-requirements.md)
- [`20-architecture-options.md`](./20-architecture-options.md)

---

## ADR-003 - MVP networking baseline: 5 GbE preferred, 2.5 GbE acceptable fallback

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware

### Context

Cluster/mesh use (2–4 nodes) and fast local RAG/data sync benefits from multi-gigabit Ethernet. Many consumer routers/switches now support at least 2.5 GbE; 5 GbE is increasingly common on higher-end boards.

### Decision

- MVP compute modules should provide **≥5 GbE** if available.
- **2.5 GbE is acceptable** for early dev boxes, but not the long-term reference.

### Consequences

- Choose boards and/or NIC options with a clear 5 GbE story.
- Mechanical design should allow a clean RJ45 opening and (optionally) a future 10 GbE variant.

---

## ADR-004 - MVP enclosure envelope: ~7 L, quiet-first airflow

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware

### Context

We want a desk/shelf appliance, not a tower. The enclosure must still fit:

- APU board (Mini-ITX-style),
- internal PSU,
- 2× NVMe,
- quiet fans and dust filtering.

### Decision

- Target enclosure volume: **~7 L** (allowed range 6–8 L).
- Design for **low-RPM, large-fan airflow** (no small high-RPM blowers).

### Consequences

- Cooling and PSU placement decisions must be validated early with physical mockups.
- Mechanical drafts must include airflow path, filters, and serviceability.

---

## ADR-005 - PSU approach: internal SFX, 500–600 W, efficiency-first

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware

### Context

APU-centric nodes have modest sustained power, but need headroom for spikes and future boards. Internal PSUs simplify the appliance UX.

### Decision

- Use an internal **SFX** PSU in the **500–600 W** class, **80+ Gold** or better.
- Prefer ATX 3.x compliant units when practical (cabling/standards alignment).

### Consequences

- PSU thermal and acoustic profile becomes part of the product experience.
- Mechanical layout must reserve space for PSU intake/exhaust and cable routing.

---

## ADR-006 - Runtime alignment requirement: close the “APU inference backend” gap

**Status:** Accepted  
**Date:** 2025-12-14  
**Owner:** @goni-hardware + @goni-software

### Context

The current kernel inference engine in-repo is HTTP vLLM client. vLLM is mature on CUDA/NVIDIA and has ROCm support for some AMD GPUs, but APU-class support must be validated (or a second backend must be added).

### Decision

- Hardware MVP remains APU-centric, **but** software must provide a validated inference backend for the APU reference hardware:
  - either validate the ROCm path for the APU target, **or**
  - implement a second backend (recommended) such as **llama.cpp** (Vulkan/HIP/CPU fallback).

### Consequences

- Hardware and software roadmaps are coupled until this is resolved.
- “Pro” nodes (NVIDIA) can serve as a temporary throughput baseline for vLLM, but must not redefine the MVP hardware envelope.

### References

- `software/kernel/goni-infer` (HttpVllmEngine)
- [`25-hardware-layers-and-supplier-map.md`](./25-hardware-layers-and-supplier-map.md)

---

## ADR-007 - Hardware-aware ITCR as a platform contract (roofline + DVFS + WAF)

**Status:** Accepted  
**Date:** 2026-01-03  
**Owner:** @goni-hardware + @goni-software

### Context

Local inference-time compute reasoning (ITCR) is constrained by memory
bandwidth, accelerator graph constraints, storage endurance, and thermal
dynamics. These constraints must be encoded as hardware platform contracts so
software scheduling and routing can enforce them.

### Decision

- Goni assumes decoding is memory-bound and routes by arithmetic intensity.
- NPUs are treated as fixed-graph accelerators with explicit shape buckets.
- Persistence MUST control write amplification via LSM-style buffering and gated
  compaction.
- Solver bursts are DVFS-clamped and duty-cycle limited.

### Consequences

- Hardware selection must provide the telemetry and knobs defined in
  `hardware/10-requirements.md`.
- Scheduling policies in `software/10-requirements.md` and
  `docs/specs/scheduler-and-interrupts.md` are mandatory for safe operation.

### Non-goals

- No hard performance numbers or thresholds in this ADR.
- Measurement plans and concrete targets are deferred to future updates.

### Failure modes and fallbacks

If telemetry is missing or unstable, Goni MUST fall back to conservative
policies: reduce duty cycle, prefer CPU/iGPU routing, and defer compaction until
safe conditions return.
