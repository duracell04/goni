---
id: GONI-IMAP-1A5127D69AE3
title: 1. Layered mental model
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Everything maps to four layers: **Silicon / Accelerators** — GPUs, NPUs, ASICs, photonic processors; what inference backends ultimately target.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/25-hardware-layers-and-supplier-map.md
  heading: 1. Layered mental model
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 1. Layered mental model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Layered mental model

Everything maps to four layers:

- **Silicon / Accelerators** — GPUs, NPUs, ASICs, photonic processors; what inference backends ultimately target.
- **Systems / Boxes** — workstations, servers, mini PCs; what sits on a desk or in a rack.
- **Edge / Always-On** — ultra-low-power inference for router/intent models that keep big compute asleep.
- **Workloads / Models** — what drives specs (LLMs, vision, embeddings, diffusion, adapters).

---
