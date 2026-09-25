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
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion; CTX-MISS-01 extends the read-side recovery contract as specified architecture requiring evaluation.
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
  - Use `CTX-MISS-01` as the bounded inference-time read-side mechanism when cognition discovers a missing evidentiary or procedural dependency after initial compilation.
- The Control Plane MUST schedule a recurring **Observation → Reflection → Planning** consolidation loop:
  - ingest raw events into episodic memory (Observation),
  - distill reflections/long-term facts (Reflection),
  - produce plans/jobs/actions using both current state and reflections (Planning).
- Council/cloud paths see only distilled/approved context unless explicitly allowed by policy.

**Rationale**

- MemGPT (Packer et al., 2023) formalises virtual context management; Goni adopts it as a kernel invariant rather than a prompt-only tool.
- Goni extends the analogy with a typed context-miss path: an active inference may request additional read-side material, but the request remains bounded by the Work Order, visibility policy, recursion limit, token budget, latency budget, and receipt requirements.
- Generative Agents (Park et al., 2023) motivates Observation–Reflection–Planning for coherent long-horizon behaviour; Goni maps that pattern into governed Control/Memory responsibilities.

**Consequence**

- Kernel APIs must surface paging syscalls to tools/agents; backlog item: expose `MEM_*` calls in the extension substrate.
- Context selection, memory paging, and context-miss recovery are tested as first-class behaviours; prompt-only hidden state is non-conformant.
- A context miss may increase available cognitive evidence but may not grant tool or execution authority.
- Nightly/periodic consolidation jobs become required workloads; they must respect policies and budgets (local-first unless configured otherwise).

---
