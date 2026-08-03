---
id: GONI-IMAP-D6E48E78EB7F
title: 5.1 MVP dashboard
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The MVP dashboard may be: A minimal web UI, A TUI/CLI summary (goni status), Or omitted entirely.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 5.1 MVP dashboard
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 5.1 MVP dashboard

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 MVP dashboard

The MVP dashboard may be:

- A minimal web UI,
- A TUI/CLI summary (goni status),
- Or omitted entirely.

MVP requirements:

- If present, it must:
  - respect UI-1..UI-5,
  - show at least a basic **Node Overview** and **Workloads** view.

The kernel must *not* depend on the dashboard; it is an optional add-on.
