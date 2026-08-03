---
id: GONI-SYNTHESIS-161FC810382A
title: Soft (UX defaults)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Coach vs Ghost modes:** Coach prompts user effort (outline/selection) before full generation; Ghost produces full drafts but flags higher cognitive offload.'
domains:
- memory
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/memory-architecture.md
  heading: Soft (UX defaults)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# Soft (UX defaults)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Soft (UX defaults)

- **Coach vs Ghost modes:** Coach prompts user effort (outline/selection) before full generation; Ghost produces full drafts but flags higher cognitive offload.
- **Active recall option:** `recall(mode=active)` returns teasers for top memories and asks the user to pick before assembling full context.
- **Attribution:** Responses tag AI-authored vs user-authored content; memory-based answers cite the source (`mem:2024-10-02 project-mtg`).
- **Spaced surfacing:** Periodic recap/review flows surface decaying memories for reinforcement.
- **Low-memory profile:** On 64 GB unified nodes, default to smaller models/tighter budgets while keeping the same UX/traceability behaviours.
