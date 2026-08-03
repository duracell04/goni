---
id: GONI-IMAP-7AD435FB079D
title: 2. What this folder contains
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 10-requirements.md The main reference for **software requirements** (capabilities, security, UX expectations, mesh behaviour, cloud usage policy).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/00-overview.md
  heading: 2. What this folder contains
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 2. What this folder contains

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. What this folder contains

- [`10-requirements.md`](/blueprint/software/10-requirements.md)
  The main reference for **software requirements** (capabilities, security, UX expectations, mesh behaviour, cloud usage policy). Any architectural proposal should be checked against these requirements.

- [`20-architecture.md`](/blueprint/software/20-architecture.md)
  (To be filled) High-level architecture for single-node service layout, multi-node / mesh topology, and how local models, data indexing, orchestration, and APIs fit together.

- `30-components/`
  Detailed notes for each major component: base OS and provisioning, orchestrator logic (routing between local and cloud), model runtime, vector database, mesh coordination, and any background workers or agents.
  - See [30-components/latent-predictor.md](/blueprint/software/30-components/latent-predictor.md) for the latent-first cognition pattern and its integration points.

- `40-apis-and-ui/`
  Description of the public API surface (e.g. chat, completion, tools), authentication and access control, and dashboard / management UI concepts.

- `50-data/`
  Data spine, planes, and TXT axiom. Start with [50-data/00-index.md](/blueprint/software/50-data/00-index.md), then [50-data/10-axioms-and-planes.md](/blueprint/software/50-data/10-axioms-and-planes.md), and [50-data/53-schema-dsl-and-macros.md](/blueprint/software/50-data/53-schema-dsl-and-macros.md) to see how the Arrow schema DSL maps into the kernel.

- [`90-decisions.md`](/blueprint/software/90-decisions.md)
  Accepted software design decisions (ADR-style). Each entry should briefly describe the choice, alternatives, and rationale.

Runnable references:
- local stack: goni-prototype-lab:deploy/docker-compose.yml
- k8s overlays: goni-prototype-lab:deploy/k8s/

---
