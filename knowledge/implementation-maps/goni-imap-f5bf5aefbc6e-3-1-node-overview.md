---
id: GONI-IMAP-F5BF5AEFBC6E
title: 3.1 Node Overview
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Shows: Uptime, version, build/commit.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 3.1 Node Overview
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.1 Node Overview

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Node Overview

Shows:

- Uptime, version, build/commit.
- CPU / GPU / NPU utilisation (current and short history).
- Memory and disk usage:
  - models,
  - Arrow data store,
  - logs.

**Source:** metrics tables and status records in Data Plane \(\mathcal{A}\).

> **Invariant UI-1 (read-only correctness)**  
> Every value shown in this view must be derivable from kernel state (metrics, status endpoints). No “UI-only” counters or guesses.

---
