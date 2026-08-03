---
id: GONI-IMAP-E2D8A099B27A
title: 0.1 Memory Plane (operational abstraction)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'While the formal tuple remains \(N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})\), we treat **long-term memory as an external, pluggable service** that the Control/Execution planes call: **API surface:** store(entry), recall(query, mode), forget(selector), summarize(range), audit(entry_id).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.1 Memory Plane (operational abstraction)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.1 Memory Plane (operational abstraction)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0.1 Memory Plane (operational abstraction)

While the formal tuple remains \(N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})\), we treat **long-term memory as an external, pluggable service** that the Control/Execution planes call:

- **API surface:** `store(entry)`, `recall(query, mode)`, `forget(selector)`, `summarize(range)`, `audit(entry_id)`.
- **Types / lifecycle:** `working` (session-scoped), `episodic` (raw events distilled over time), `semantic` (facts with decay/pin), `procedural` (versioned skills/tools). States move `raw -> distilled -> archived/tombstoned`, with decay on importance/access.
- **Local-first:** long-term memory lives on-device (Arrow + vector/graph backend). Council/cloud paths see only distilled facts or session context unless explicitly allowed.
- **Reasoning statelessness:** LLM engines stay stateless; they access memory exclusively through this plane so backends can be swapped (Qdrant/Arrow today; OpenMemory/Mem0/curved indexes later) without changing \(\mathcal{E}\).

---
