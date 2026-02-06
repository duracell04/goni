# Goni Memory Architecture (Legacy Draft)

> **Status:** Legacy UX concept (kept for historical context).
>
> **Source of truth for current architecture:**
> - Kernel-level latent state: `blueprint/30-specs/latent-state-contract.md` (LSC-01)
> - Plane model and payload types: `blueprint/software/50-data/10-axioms-and-planes.md`
> - MVP schema tables: `blueprint/software/50-data/51-schemas-mvp.md`
>
> This document predates the Latent State Contract and uses an older, UX-first framing
> (working/episodic/semantic/procedural). It is useful for product thinking, but it is not
> the normative contract for implementation.

This note captures the **hard** and **soft** commitments for Goni's long-term memory so it strengthens users rather than replacing their thinking.

## Hard (architecture)

- **Memory Plane API:** `store(entry)`, `recall(query, mode)`, `forget(selector)`, `summarize(range)`, `audit(entry_id)`; engines/controllers use this interface only.
- **Types & lifecycle:** `working` (session), `episodic` (raw -> distilled -> archived/tombstoned), `semantic` (facts with decay/pin), `procedural` (versioned skills). States carry `importance` that decays unless reinforced.
- **Local-first:** Long-term memory stays on-device (Arrow + vector/graph backend). Council/cloud paths see at most distilled facts or session context when explicitly allowed.
- **Backend-pluggable:** Default is Arrow + Qdrant/Lance; backend can be swapped (e.g. OpenMemory/Mem0/curved index) if it preserves the API and lifecycle semantics.
- **Traceability & redaction:** Recall returns waypoints (`why this memory`); `forget` performs graph-aware redaction and reindexing.

## Soft (UX defaults)

- **Coach vs Ghost modes:** Coach prompts user effort (outline/selection) before full generation; Ghost produces full drafts but flags higher cognitive offload.
- **Active recall option:** `recall(mode=active)` returns teasers for top memories and asks the user to pick before assembling full context.
- **Attribution:** Responses tag AI-authored vs user-authored content; memory-based answers cite the source (`mem:2024-10-02 project-mtg`).
- **Spaced surfacing:** Periodic recap/review flows surface decaying memories for reinforcement.
- **Low-memory profile:** On 64 GB unified nodes, default to smaller models/tighter budgets while keeping the same UX/traceability behaviours.

