---
id: GONI-IMAP-4AA7A8D52578
title: 6. Versioning and stability
type: implementation-map
status: draft
implementation_state: specified_only
proposition: All endpoints are under the /v1 prefix.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 6. Versioning and stability
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 6. Versioning and stability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Versioning and stability

All endpoints are under the /v1 prefix.

> **Invariant API-5 (v1 semantic stability)**
> For any request \(r \in \mathsf{Req}_{\text{v1}}\) that was valid at time \(t_0\), its meaning under /v1 at time \(t_1 \ge t_0\) must not change in a way that breaks well-behaved clients (no backwards-incompatible type/behaviour changes).

Evolution rules:

* Adding new **optional** fields is allowed.
* Adding new **endpoints** beside /v1/chat/completions is allowed.
* Changing semantics of existing fields or removing them requires a new version prefix (e.g. /v2).

---
