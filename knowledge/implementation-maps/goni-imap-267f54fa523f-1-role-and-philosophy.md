---
id: GONI-IMAP-267F54FA523F
title: 1. Role and philosophy
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The dashboard is an **optional client** of the Goni node.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 1. Role and philosophy
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 1. Role and philosophy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role and philosophy

The dashboard is an **optional client** of the Goni node. The kernel is fully usable without it.

Its purposes are:

- Make internal state (health, queues, RAG behaviour) visible.
- Provide a *safe* control panel for a few high-level actions (e.g. cancel job, toggle RAG).

> **Principle:** The dashboard is a mirror, not a second brain.  
> It reflects state and invokes documented APIs; it never introduces its own “shadow state”.

---
