---
id: GONI-PRINCIPLE-8E2E987DEBFA
title: Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: All IDs are fixed_size_binary[16] (UUIDv7) and equal Spine.row_id for their table.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: Invariants
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Invariants
- All IDs are `fixed_size_binary[16]` (UUIDv7) and equal `Spine.row_id` for their table.
- No `LargeUtf8` outside `Chunks.text` and `Prompts.text` (TXT axiom).
- Visual asset rows store metadata and content-addressed refs only; raw image
  binaries, masks, and full OCR text are not stored in Control-plane records or
  receipts.
- Dictionaries enumerate finite vocabularies; adding a new label/value requires schema version bump.
- Latent state, audit, and manifest records include `agent_id`, `policy_hash`,
  `state_snapshot_id`, and `provenance` (directly or by reference).
