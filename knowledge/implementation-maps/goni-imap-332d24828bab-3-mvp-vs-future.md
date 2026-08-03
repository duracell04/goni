---
id: GONI-IMAP-332D24828BAB
title: 3. MVP vs future
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'For the MVP / prototype: The **HTTP API** is **mandatory** – a node is “alive” if and only if it serves /v1/chat/completions correctly.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/00-index.md
  heading: 3. MVP vs future
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 3. MVP vs future

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. MVP vs future

For the MVP / prototype:

- The **HTTP API** is **mandatory** – a node is “alive” if and only if it serves /v1/chat/completions correctly.
- The **dashboard** is **optional** – the kernel must be fully usable without any UI running.

Future work (admin APIs, rich dashboards, multi-user UIs) will extend this directory but must not violate the invariants defined here.
