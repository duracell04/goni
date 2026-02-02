# 40.20 – Dashboard & UI Concepts

Status: v0.1 (2025-11-30)  
Scope: Conceptual operator UI for a single node  
Audience: Designers, systems engineers, power users

---

## 1. Role and philosophy

The dashboard is an **optional client** of the Goni node. The kernel is fully usable without it.

Its purposes are:

- Make internal state (health, queues, RAG behaviour) visible.
- Provide a *safe* control panel for a few high-level actions (e.g. cancel job, toggle RAG).

> **Principle:** The dashboard is a mirror, not a second brain.  
> It reflects state and invokes documented APIs; it never introduces its own “shadow state”.

---

## 2. Personas

Two primary personas guide the MVP:

- **Owner / operator**
  - Runs the node on their laptop or home server.
  - Cares about resource usage, model status, and “is my node healthy?”.

- **Developer / power user**
  - Builds apps on top of the API.
  - Uses the dashboard to debug prompts, RAG, and scheduling behaviour.

We do **not** assume multiple concurrent end-users or fine-grained RBAC for the MVP.

---

## 3. Views and their plane mappings

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

### 3.3 Data & RAG

Gives a mental model of “what the node knows” and how RAG behaves:

- Connected sources (file roots, mailboxes, etc. – even if conceptual in MVP).
- Counts:
  - documents,
  - chunks,
  - index sizes.
- For a sample query:
  - retrieved candidates from VecDB (with similarity),
  - selected subset \(S\) used in the actual context.

**Source:** VecDB + Context Plane \(\mathcal{X}\) logging.

> **Invariant UI-3 (context transparency)**  
> Any explanation like “this chunk was selected because …” must be backed by actual data from:
> - VecDB search results (similarity scores), and
> - the selector’s choice set \(S\) and objective contributions.  
> The UI cannot invent explanations that the kernel cannot justify.

---

### 3.4 Models & Settings

Shows:

- List of available models (goni-small, goni-large, …).
- Per-model capabilities (max context length, approximate throughput, memory footprint).
- Which backend is used (llama.cpp, vLLM, etc.).
- High-level toggles, e.g.:
  - “local-only mode” (no network use),
  - “RAG enabled by default”,
  - “default model tier for interactive jobs”.

Settings changes must map to documented configuration mechanisms (config files or dedicated APIs).

---

## 4. Interaction model

The dashboard communicates with the node **only via APIs**:

- Public HTTP API (/v1/*), and
- Future admin API (/v1/admin/*, once it exists).

It does **not** reach into the kernel via private sockets, shared memory, or direct DB access.

In token-auth mode, the dashboard typically uses an owner role; in local-trust mode, it inherits the trust of the local OS user.

> **Invariant UI-4 (API completeness)**  
> Any state change that the dashboard can perform (e.g. cancelling a job, toggling a flag) must be reproducible by scripted API calls. The UI may not perform opaque modifications that clients cannot replicate.

---

## 5. MVP vs future UI

### 5.1 MVP dashboard

The MVP dashboard may be:

- A minimal web UI,
- A TUI/CLI summary (goni status),
- Or omitted entirely.

MVP requirements:

- If present, it must:
  - respect UI-1..UI-4,
  - show at least a basic **Node Overview** and **Workloads** view.

The kernel must *not* depend on the dashboard; it is an optional add-on.

### 5.2 Future directions

Later versions may add:

- Multi-node / mesh topology view,
- Historical charts of scheduler metrics and RAG quality,
- Per-user profiles with limited permissions,
- Plugins and tool-introspection UIs.

These are explicitly future scope and will be specified only after the corresponding APIs and data are in place.

---

## 6. Non-goals

The dashboard is **not**:

- A full IDE or editor,
- A replacement for external observability stacks (Prometheus, Grafana),
- A GUI for arbitrary database operations on \(\mathcal{A}\).

Its job is to provide **truthful, high-level insight** and a **small set of safe controls** – nothing more.
