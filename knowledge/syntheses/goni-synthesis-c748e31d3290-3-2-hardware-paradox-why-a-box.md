---
id: GONI-SYNTHESIS-C748E31D3290
title: 3.2 Hardware paradox (“why a box?”)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Target users often already own: powerful laptops (M-series Mac, Ryzen AI), idle mini-PCs or NAS boxes.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 3.2 Hardware paradox (“why a box?”)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3.2 Hardware paradox (“why a box?”)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Hardware paradox (“why a box?”)

- Target users often already own:
  - powerful laptops (M-series Mac, Ryzen AI),
  - idle mini-PCs or NAS boxes.
- Dedicated hardware:
  - risks being seen as expensive, rapidly obsolete, e-waste.
  - adds logistics: manufacturing, returns, certification, shipping, power-cultural differences.

**Risk:**  
Perception of hardware lock-in or “yet another device” kills adoption, especially for the homelab crowd.

**Mitigation path:**

- **Position the box as a trust and continuity anchor**, not just compute:
  - holds secrets, raw data, audit logs, backup logic.
- **Offer BYO hardware image** for advanced users (later), without undermining the main appliance story.
- **Design for migration** – box → box upgrades are easy, reducing e-waste anxiety.

---
