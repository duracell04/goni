---
id: GONI-DECISION-FEB39440C565
title: D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** The Memory/Context planes MUST implement **virtual context management** at the kernel level: Treat the prompt window as RAM and external stores (Arrow spine + vector/graph backends) as Disk.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-017 - Virtual context management + consolidation loop (MemGPT / Generative Agents)

**Formal statement**

- The Memory/Context planes MUST implement **virtual context management** at the kernel level:
  - Treat the prompt window as RAM and external stores (Arrow spine + vector/graph backends) as Disk.
  - Expose explicit paging/syscalls (e.g. `MEM_READ`, `MEM_WRITE`, `MEM_SUMMARIZE`, `MEM_FORGET`) to move data across tiers.
  - Keep LLM engines stateless; all long-lived state flows through the Memory Plane.
- The Control Plane MUST schedule a recurring **Observation → Reflection → Planning** consolidation loop:
  - ingest raw events into episodic memory (Observation),
  - distill reflections/long-term facts (Reflection),
  - produce plans/jobs/actions using both current state and reflections (Planning).
- Council/cloud paths see only distilled/approved context unless explicitly allowed by policy.

**Rationale**

- MemGPT (Packer et al., 2023) formalises virtual context management; Goni adopts it as a kernel invariant rather than a prompt-only tool.
- Generative Agents (Park et al., 2023) shows Observation–Reflection–Planning is necessary for coherent long-horizon behaviour; we bake this into the Control/Memory planes.

**Consequence**

- Kernel APIs must surface paging syscalls to blueprint/tools/agents; backlog item: expose `MEM_*` calls in the extension substrate.
- Context selection and memory paging are tested as first-class behaviours; prompt-only hacks are non-conformant.
- Nightly/periodic consolidation jobs become required workloads; they must respect policies and budgets (local-first unless configured otherwise).

---
