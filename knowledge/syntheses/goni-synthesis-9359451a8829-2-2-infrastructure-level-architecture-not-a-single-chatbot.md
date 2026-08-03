---
id: GONI-SYNTHESIS-9359451A8829
title: 2.2 Infrastructure-level architecture, not a single chatbot
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Kernel + Planes** Data Plane (Arrow spine), Context Plane (KV paging / selection), Control Plane (scheduler / jobs).'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 2.2 Infrastructure-level architecture, not a single chatbot
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2.2 Infrastructure-level architecture, not a single chatbot

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Infrastructure-level architecture, not a single chatbot

- **Kernel + Planes**  
  - Data Plane (Arrow spine), Context Plane (KV paging / selection), Control Plane (scheduler / jobs).
- **System invariants**  
  - Shared, columnar memory instead of JSON/bespoke DB per feature.  
  - Typed, permissioned jobs instead of ad-hoc scripts.

**Why this matters:**  
Most competitors are “UI → LLM → ad-hoc store”. Goni positions itself as the **infrastructure layer** others could eventually build on.

---
