---
id: GONI-DECISION-5E17F1A4D2EE
title: D-016 - Memory Plane as a Pluggable, Local-First Service
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** Long-term memory is modelled as a separate **Memory Plane** with a stable API (store, recall, forget, summarize, audit).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-016 - Memory Plane as a Pluggable, Local-First Service
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-016 - Memory Plane as a Pluggable, Local-First Service

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-016 - Memory Plane as a Pluggable, Local-First Service

**Formal statement**

Long-term memory is modelled as a separate **Memory Plane** with a stable API (`store`, `recall`, `forget`, `summarize`, `audit`). Engines and controllers access memory only through this plane; core LLMs stay stateless. The default backend is Arrow tables + vector index (Qdrant/Lance), and backends are **swappable** (e.g. OpenMemory/Mem0/graph/curved indexes) without changing \((\mathcal{E})\).

**Rationale**

- Keeps reasoning and memory decoupled; enables backend experimentation without kernel surgery.  
- Aligns with privacy/local-first: long-term memory stays on-device; council/cloud paths see at most distilled facts or session context by explicit choice.  
- Supports lifecycle (working/episodic/semantic/procedural) with decay/pin/forget and auditability.

**Consequence**

- Kernel code and tooling MUST use the Memory Plane interface rather than ad-hoc embedding stores.  
- Forget/redaction and audit traces are first-class behaviours of the plane.  
- Backend swaps must preserve the API contract and lifecycle semantics; otherwise the decision must be amended.

---
