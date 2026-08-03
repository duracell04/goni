---
id: GONI-IMAP-B9113F5DE481
title: 7. Non-goals for MVP
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The v1 API explicitly does **not** attempt to: Expose admin / metrics endpoints (that will be a separate “admin API”).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 7. Non-goals for MVP
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 7. Non-goals for MVP

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Non-goals for MVP

The v1 API explicitly does **not** attempt to:

* Expose admin / metrics endpoints (that will be a separate “admin API”).
* Handle multimodal inputs (audio, images) – can be added later as separate endpoints.
* Provide multi-tenant isolation; the MVP assumes a single trust domain per node.

The intent is a **small, stable surface** that can be relied on locally, while leaving room for future extensions.
