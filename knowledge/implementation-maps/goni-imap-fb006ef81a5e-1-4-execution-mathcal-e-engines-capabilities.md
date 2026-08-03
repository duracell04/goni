---
id: GONI-IMAP-FB006EF81A5E
title: 1.4 Execution \(\mathcal{E}\) – Engines & Capabilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Object.** A family of models \(\mathcal{M}\) and tools running in an **effectful extension** of \(\mathcal{A}\): Engines \(m \in \mathcal{M}\) with capability descriptors.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 1.4 Execution \(\mathcal{E}\) – Engines & Capabilities
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 1.4 Execution \(\mathcal{E}\) – Engines & Capabilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.4 Execution \(\mathcal{E}\) – Engines & Capabilities

**Object.**  
A family of models \(\mathcal{M}\) and tools running in an **effectful extension** of \(\mathcal{A}\):

- Engines \(m \in \mathcal{M}\) with capability descriptors.  
- Wasm modules \(W\) with capability sets \(\mathsf{Cap}(W)\).  
- An effectful category \(\mathcal{A}^\mathsf{eff}\) where side effects are annotated capabilities.

**Invariant E1 (capability safety).**

For any effectful morphism \(f_W\) implemented by a Wasm module \(W\),
$$
\mathsf{Effects}(f_W) \subseteq \mathsf{Cap}(W),
$$
and by default \(\mathsf{Cap}(W)\) is **local-only** (no network) for core tools.

**Invariant E2 (local-first).**

The core request?response function
$$
\mathsf{Run} : \mathsf{Req} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}
$$
is total using only local state and compute; any network I/O is non-essential and opt-in.

**Proof obligation (theoretical).**

- Exhibit the host capability interface as a typed API.  
- Argue that all implemented tools / engines factor through this interface, so capability sets are sound.

**Empirical check (MVP).**

- Provide at least one **offline test configuration** where:
  - All network capabilities are disabled, yet  
  - Chat + RAG work against local models and local data.  
- Run that configuration in CI or as a nightly test.

**Invariant E3 (deterministic self-loop).**

When a request is marked deterministic, the engine uses the deterministic preset (single worker/thread, batch size 1, no continuous batching, TF32 off on NVIDIA, seed if supported). Empirical check:

- Run a fixed prompt in a self-loop for \(N\) steps (e.g. \(N=128\)) under the deterministic profile twice.
- Assert identical token streams (bitwise) and log the backend blueprint/hardware/driver version used.
- Fail conformance if drift appears.

---
