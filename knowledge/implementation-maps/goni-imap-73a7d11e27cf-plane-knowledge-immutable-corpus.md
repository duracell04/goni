---
id: GONI-IMAP-73A7D11E27CF
title: Plane 𝒜 – Knowledge (immutable corpus)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Concepts: Doc, Chunk, Embedding; specified-only visual extensions add VisualAsset and VisualAssetDerivation.'
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
  heading: Plane 𝒜 – Knowledge (immutable corpus)
  revision: dcbe5931107b72f6a6af295e9e1b943accb6a2f9
---

# Plane 𝒜 – Knowledge (immutable corpus)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Plane 𝒜 – Knowledge (immutable corpus)
- Concepts: `Doc`, `Chunk`, `Embedding`; specified-only visual extensions add `VisualAsset` and `VisualAssetDerivation`.
- Tables: Docs, Chunks, Embeddings, StateSnapshots, StateDeltas, LatentSummaries, MemoryEntries. Specified-only extensions: VisualAssets, VisualAssetDerivations.
- Allowed FK targets from other planes: `Chunks.chunk_id` (referenced by 𝒳.ContextItems), `Embeddings.chunk_id`, and `VisualAssets.visual_asset_id` after the visual schema extension is implemented.
- Forbidden: no inbound FK from ℰ to `text`; no raw image binaries in Control-plane records; no mutable state.
