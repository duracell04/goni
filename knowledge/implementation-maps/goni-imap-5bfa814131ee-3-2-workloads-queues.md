---
id: GONI-IMAP-5BFA814131EE
title: 3.2 Workloads & Queues
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Visualises the Control Plane \(\mathcal{K}\): Queue lengths per TaskClass (interactive, background, maintenance).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 3.2 Workloads & Queues
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.2 Workloads & Queues

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Workloads & Queues

Visualises the Control Plane \(\mathcal{K}\):

- Queue lengths per TaskClass (interactive, background, maintenance).
- Aggregate latencies (p50/p95/p99) for recent interactive jobs.
- List of running / queued jobs with:
  - class,
  - age,
  - rough size (tokens/budget).

Job lifecycle:

$$
\text{submitted} \to \text{queued} \to \text{running} \to \{\text{succeeded}, \text{failed}, \text{cancelled}\}.
$$

**Allowed action:** cancel(job_id) → translated into the same API call any client could use.

> **Invariant UI-2 (no 𝒦 bypass)**  
> The dashboard must not manipulate scheduler state directly. It can only request actions (cancel, reprioritise if supported) through public kernel APIs.

---
