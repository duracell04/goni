---
id: GONI-DECISION-0532C27DA3CC
title: D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** All work units (LLM calls, embeddings, indexing, compaction) are represented as jobs in the queueing network \(\mathcal{K}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter

**Formal statement**

All work units (LLM calls, embeddings, indexing, compaction) are represented as jobs in the queueing network \(\mathcal{K}\). No component is allowed to maintain a “hidden” unbounded queue outside \(\mathcal{K}\).

Scheduling decisions are made exclusively by Policy K1 (MaxWeight) over classes 1–3.

**Rationale**

- Allows use of queueing theory to prove stability (Theorem 3.1).  
- Prevents “queueing inside the queue” anti-patterns that make latencies opaque.

**Consequence**

- Libraries like LLM engines or indexers must expose backpressure / job API so that their work can be scheduled centrally.  
- Introducing a new class of long-running task requires updating \(\mathcal{K}\) and its invariants, not spinning up an ad-hoc thread pool.

---
