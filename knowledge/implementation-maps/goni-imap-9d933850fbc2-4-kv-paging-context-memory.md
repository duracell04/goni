---
id: GONI-IMAP-9D933850FBC2
title: 4. KV Paging (Context Memory)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Inputs: 𝒳.ContextItems plus internal page table.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/52-zero-copy-mechanics.md
  heading: 4. KV Paging (Context Memory)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4. KV Paging (Context Memory)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. KV Paging (Context Memory)
- Inputs: 𝒳.ContextItems plus internal page table.
- Process: update residency flags; evict without touching `Chunks.text`.
- Copies: 0.
