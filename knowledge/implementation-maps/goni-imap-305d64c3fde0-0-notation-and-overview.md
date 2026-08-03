---
id: GONI-IMAP-305D64C3FDE0
title: 0. Notation and overview
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'A **Goni node** is modelled as a 4-tuple $$ N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E}) $$ where: \(\mathcal{A}\): **Arrow Spine** â€“ a symmetric monoidal category of zero-copy data transforms.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0. Notation and overview
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0. Notation and overview

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0. Notation and overview

A **Goni node** is modelled as a 4-tuple
$$
N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})
$$
where:

- \(\mathcal{A}\): **Arrow Spine** â€“ a symmetric monoidal category of zero-copy data transforms.
- \(\mathcal{X}\): **Context Plane** â€“ a submodular optimisation problem over retrieved chunks.
- \(\mathcal{K}\): **Control Plane** â€“ a controlled queueing network with a Lyapunov scheduler.
- \(\mathcal{E}\): **Execution Substrate** â€“ LLM / embed engines + Wasm sandboxes.

We use this decomposition both as an implementation guide and as the basis for our invariants.

**How to read this doc.**

- This file defines the architecture and its formal objects.
- `blueprint/software/30-conformance.md` turns those objects into invariants and proof obligations (what must be shown or tested for an MVP node to be â€œconformantâ€).
- `blueprint/software/95-theory-appendix.md` gives a brief theoretical backdrop (category theory, submodularity, Lyapunov stability, bandits, capabilities).

Read in the order: **Architecture (20) -> Conformance (30) -> Theory (95)**.

---
