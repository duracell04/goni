---
id: GONI-SPEC-33294E0D3306
title: 3.3 Memory & Cognition
type: specification
status: draft
implementation_state: specified_only
proposition: 'The node should treat **long-term memory as a separate plane** with explicit lifecycle: working/session context is transient, episodic history is distilled over time, semantic facts persist with decay and can be pinned, procedural knowledge is versioned.'
domains:
- software
aliases: []
relations:
- type: refined_by
  target: DREAM-01
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.3 Memory & Cognition
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.3 Memory & Cognition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Memory & Cognition

- The node should treat **long-term memory as a separate plane** with explicit lifecycle:
  - working/session context is transient,
  - episodic history is distilled over time,
  - semantic facts persist with decay and can be pinned,
  - procedural knowledge is versioned.
- Versioned procedural knowledge that is intended to control repeated task execution is governed through `SKILL-REG-01`. Durable personal procedural memories may reference governed skills, but memory content does not silently become executable instruction.
- The Memory/Context planes must implement **virtual context management** (MemGPT-style):
  - prompt/context window as RAM, Arrow/vector/graph stores as Disk,
  - explicit paging/syscalls (`MEM_READ`, `MEM_WRITE`, `MEM_SUMMARIZE`, `MEM_FORGET`) for moving data across tiers,
  - LLM engines stay stateless; all long-lived state flows through the Memory Plane.
- The Control Plane must run a **consolidation loop** (Observation → Reflection → Planning):
  - ingest raw events into episodic memory,
  - distill evidence-backed summaries/facts and explicitly typed derived
    candidates periodically (e.g. nightly/weekly),
  - plan/schedule actions using current state and governed reflections.
- Periodic cognition may extend consolidation with governed offline synthesis
  under `DREAM-01`. Such cycles may derive abstractions, associations,
  contradictions, counterfactuals, simulations, evidence requests, or
  hypotheses from authorized memory. Synthetic outputs remain explicitly
  distinguishable from observed or independently confirmed facts and do not
  silently become executable procedural knowledge, policy, preferences, or
  authority state.
- The existing `MemoryEntries` substrate remains canonical for durable dream
  outputs in v1. New synthetic entries use `hypothesis` or `derived` and
  existing provenance, TTL/review, conflict, and quarantine semantics rather
  than introducing a second hypothesis store.
- The system must support **local-only long-term memory** by default; cloud/council access is limited to distilled facts or session context unless explicitly allowed.
- To avoid **cognitive offloading debt**, default UX for learning/creative flows should:
  - prompt user effort (outline/selection) before full generation,
  - attribute which parts were AI- vs user-authored,
  - expose which memories were retrieved and why (traceable recall).
- **Supported minimum vs reference build:** the software should run on lower-memory hardware (e.g. **64 GB unified**) by tightening context budgets, model sizes, and cache policies while preserving behaviour. However, the **product reference build** for the MVP story is **128 GB unified memory** (see `blueprint/hardware/90-decisions.md` ADR-002).

---
