---
id: GONI-SYNTHESIS-3269DCF51EB9
title: What Goni does differently (by design)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'The **Goni MVP** concept (as defined elsewhere in this repo) is intentionally narrower and more product-oriented: Goni assumes a **strong local node** (small appliance with a high-end APU and 128 GB unified memory), not “any old device you can find”.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: What Goni does differently (by design)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# What Goni does differently (by design)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### What Goni does differently (by design)

The **Goni MVP** concept (as defined elsewhere in this repo) is intentionally narrower and more product-oriented:

- Goni assumes a **strong local node** (small appliance with a high-end APU and 128 GB unified memory),  
  not “any old device you can find”.

- Goni is not primarily a **research platform for arbitrary clusters**; it is a **personal AI appliance** with:
  - local-first assistant,  
  - RAG on personal data,  
  - optional multi-node mesh,  
  - optional integration with heavier nodes (e.g. Grace Blackwell GN100).

- Goni’s success is measured less by “how big a model can we push over Wi-Fi” and more by:
  - **latency and reliability for day-to-day assistant tasks**,  
  - **privacy guarantees**,  
  - **ease of setup and operation** for non-experts.

In that sense, you can think of Goni as:

> sitting one layer **above** EXO / Cake / prima.cpp / llama.cpp,  
> borrowing their insights and sometimes their runtimes,  
> but wrapping them into a **deliberately opinionated, appliance-grade product**.

---
