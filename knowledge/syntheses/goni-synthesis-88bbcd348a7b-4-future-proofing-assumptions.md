---
id: GONI-SYNTHESIS-88BBCD348A7B
title: 4. Future-proofing assumptions
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'We design Goni MVP with the following in mind: **APU roadmap**: future AMD/Intel APUs with more NPU TOPS and better FP8/FP4 -> we should be able to swap in a new mainboard without changing the Goni case, PSU, or overall architecture.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 4. Future-proofing assumptions
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 4. Future-proofing assumptions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Future-proofing assumptions

We design Goni MVP with the following in mind:

- **APU roadmap**: future AMD/Intel APUs with more NPU TOPS and better FP8/FP4 -> we should be able to swap in a new mainboard without changing the Goni case, PSU, or overall architecture.

- **NPU evolution**: NPUs become real inference backends for smaller models (ASR, vision, routing). The orchestrator must treat NPUs as first-class targets, not afterthoughts.

- **GN100 / Blackwell-class nodes**:
  - GN100 today: 128 GB unified memory, 1 PFLOP FP4, 0.5 L form factor.
  - Future GBxx mini-DGX successors can be added as **"Goni Max nodes"** in the mesh, without redesigning the base Goni.

The blueprint and prototypes remain open-source; the funded MVP will build from this public spec, not a private fork.

---
