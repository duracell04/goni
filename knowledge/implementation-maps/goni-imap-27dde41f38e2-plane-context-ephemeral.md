---
id: GONI-IMAP-27DDE41F38E2
title: Plane 𝒳 – Context (ephemeral)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Concepts: live prompt text and selected retrieval units.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/30-plane-contracts.md
  heading: Plane 𝒳 – Context (ephemeral)
  revision: dcbe5931107b72f6a6af295e9e1b943accb6a2f9
---

# Plane 𝒳 – Context (ephemeral)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Plane 𝒳 – Context (ephemeral)
- Concepts: live prompt text and selected retrieval units.
- Tables: Prompts, ContextItems.
- Allowed FK targets: `context_id` may be referenced by ℰ.LlmCalls; `chunk_id` references 𝒜.Chunks.
- Forbidden: persistence beyond retention window; no sharing of raw text outside 𝒳.
