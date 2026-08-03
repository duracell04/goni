---
id: GONI-SPEC-90EA3B175CC1
title: 9. Explicit Non-Goals (Software MVP)
type: specification
status: draft
implementation_state: specified_only
proposition: 'For the MVP, Goni software is **not** intended to be: A multi-tenant platform hosting unrelated users or organisations.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 9. Explicit Non-Goals (Software MVP)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 9. Explicit Non-Goals (Software MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Explicit Non-Goals (Software MVP)

For the MVP, Goni software is **not** intended to be:

- A multi-tenant platform hosting unrelated users or organisations.
- A general-purpose container hosting platform for arbitrary workloads.
- A fully-fledged replacement for enterprise MLOps platforms.

These may be future directions, but the MVP focuses on **one owner (or small team) per Goni cluster**, with a strong emphasis on local-first personal AI.
