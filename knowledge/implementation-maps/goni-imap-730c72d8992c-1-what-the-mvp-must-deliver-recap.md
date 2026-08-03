---
id: GONI-IMAP-730C72D8992C
title: 1. What the MVP must deliver (recap)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The MVP node must: Fit a **small, quiet appliance** envelope (target ~7 L, allowed 6???8 L).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 1. What the MVP must deliver (recap)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1. What the MVP must deliver (recap)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. What the MVP must deliver (recap)

The MVP node must:

- Fit a **small, quiet appliance** envelope (target ~7 L, allowed 6???8 L).
- Run **two local OSS models in parallel** (typical: 8???14B 4???5-bit quant) plus RAG indexing.
- Sustain a **domestic power + acoustics** profile (few hundred watts max; quiet under interactive use).
- Support **cluster/mesh** operation over Ethernet (2???4 nodes without special switching).
- Be upgradeable by **swapping the compute module** without redesigning the whole enclosure.

---
