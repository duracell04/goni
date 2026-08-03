---
id: GONI-IMAP-F2312100B325
title: 4. Interaction model
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The dashboard communicates with the node **only via APIs**: Public HTTP API (/v1/*), and Future admin API (/v1/admin/*, once it exists).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 4. Interaction model
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 4. Interaction model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Interaction model

The dashboard communicates with the node **only via APIs**:

- Public HTTP API (/v1/*), and
- Future admin API (/v1/admin/*, once it exists).

It does **not** reach into the kernel via private sockets, shared memory, or direct DB access.

In token-auth mode, the dashboard typically uses an owner role; in local-trust mode, it inherits the trust of the local OS user.

> **Invariant UI-5 (API completeness)**  
> Any state change that the dashboard can perform (e.g. cancelling a job, toggling a flag) must be reproducible by scripted API calls. The UI may not perform opaque modifications that clients cannot replicate.

---
