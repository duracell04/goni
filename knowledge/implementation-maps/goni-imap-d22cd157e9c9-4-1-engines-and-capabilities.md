---
id: GONI-IMAP-D22CD157E9C9
title: 4.1 Engines and capabilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Let \(\mathcal{M}\) be the set of models (LLMs, embedding models, classifiers).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 4.1 Engines and capabilities
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 4.1 Engines and capabilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Engines and capabilities

Let \(\mathcal{M}\) be the set of models (LLMs, embedding models, classifiers). For each model \(m \in \mathcal{M}\) we define capability descriptors:

- Max context length \(L_m\).  
- Approximate throughput \(\theta_m\) (tokens/s).  
- Memory footprint \(R_m\) (RAM/VRAM).  
- Supported quantisations, devices, etc.

In code this is a struct:

```rust
pub struct Capability {
    pub max_ctx: usize,
    pub throughput_toks_per_s: f32,
    pub mem_bytes: u64,
    pub device: DeviceKind,
    // ...
}
```

The Control Plane queries these capabilities via a total function:
$$
\mathsf{cap} : \mathcal{M} \to \mathsf{Capability}.
$$
