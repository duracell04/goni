---
id: GONI-IMAP-47F97860D4F4
title: 6. Non-goals
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The dashboard is **not**: A full IDE or editor, A replacement for external observability stacks (Prometheus, Grafana), A GUI for arbitrary database operations on \(\mathcal{A}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 6. Non-goals
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 6. Non-goals

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Non-goals

The dashboard is **not**:

- A full IDE or editor,
- A replacement for external observability stacks (Prometheus, Grafana),
- A GUI for arbitrary database operations on \(\mathcal{A}\).

Its job is to provide **truthful, high-level insight** and a **small set of safe controls** – nothing more.
