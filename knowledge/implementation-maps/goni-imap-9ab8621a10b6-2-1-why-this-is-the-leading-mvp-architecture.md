---
id: GONI-IMAP-9AB8621A10B6
title: 2.1 Why this is the leading MVP architecture
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'An APU-centric node (CPU + iGPU + NPU + unified LPDDR5X) is currently the best match for: compact enclosure and quiet cooling, ???enough???'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 2.1 Why this is the leading MVP architecture
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2.1 Why this is the leading MVP architecture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Why this is the leading MVP architecture

An APU-centric node (CPU + iGPU + NPU + unified LPDDR5X) is currently the best match for:

- compact enclosure and quiet cooling,
- ???enough??? GPU acceleration for quantised inference,
- large unified memory for model + KV cache + embeddings,
- low integration complexity (single compute board + standard PSU + NVMe).

The key requirement for this architecture to feel ???real??? is **128 GB unified memory**.
