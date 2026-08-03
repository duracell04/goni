---
id: GONI-IMAP-6B28A77ADB85
title: 2. Personas
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Two primary personas guide the MVP: **Owner / operator** Runs the node on their laptop or home server.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 2. Personas
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 2. Personas

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Personas

Two primary personas guide the MVP:

- **Owner / operator**
  - Runs the node on their laptop or home server.
  - Cares about resource usage, model status, and “is my node healthy?”.

- **Developer / power user**
  - Builds apps on top of the API.
  - Uses the dashboard to debug prompts, RAG, and scheduling behaviour.

We do **not** assume multiple concurrent end-users or fine-grained RBAC for the MVP.

---
