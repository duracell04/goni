# Goni Software Requirements (MVP)

> This document describes what the **final Goni product** should be able to do from a software and system-behaviour perspective.  
> It is intentionally technology-agnostic and avoids naming specific frameworks, vendors, or libraries.

---

## 1. Purpose

The Goni software stack turns the hardware node into:

- a **local-first personal AI assistant**,
- a **data hub** for the user’s own documents, email, and other sources,
- a **cluster node** that can cooperate with other Goni nodes,
- a **gateway** to external AI services, used only when they add real value.

The software must balance **performance**, **privacy**, and **simplicity**.

---

## 2. Core Capabilities

At a minimum, a single Goni node should:

1. Provide an **interactive conversational assistant** with:
   - natural language chat,
   - optional voice input and output (when peripherals are available),
   - memory of context within a session.

2. Support **retrieval-augmented generation (RAG)** over:
   - local documents and notes,
   - emails and calendar entries (if connected),
   - other user-approved data sources.

3. Offer a **coding assistant** behaviour:
   - explanation of code,
   - basic code generation,
   - summarisation of diffs / pull requests,
   - without requiring external cloud access for most tasks.

4. Perform **lightweight model personalisation**:
   - training small adapters or similar techniques on user data,
   - within the compute limits of the device,
   - without full retraining of large base models.

5. Expose a **network API** suitable for:
   - integration with local tools (editors, terminals, browsers),
   - remote access from the user’s other devices (laptop, phone, tablet).

---

## 3. Local AI Behaviour

### 3.1 Local-First Policy

- The system should prefer **local model inference** whenever reasonably possible, for:
  - chat and reasoning,
  - summarisation,
  - coding help,
  - RAG responses.

- Local models should be:
  - Loaded, managed, and swapped as needed within the available hardware capacity.
  - Configurable (e.g. user can opt into a “bigger but slower” local model or a “smaller but faster” one).

### 3.2 Resource Awareness

- The system must:
  - Monitor its own CPU, memory, storage, and accelerator utilisation.
  - Avoid overloading the device to the point of becoming unresponsive.
  - Degrade gracefully under high load (e.g. queue tasks, slow down background jobs).
  - Provide a **deterministic inference mode** for audit/self-loop workloads:
    - temperature = 0, fixed seed, batch size 1, no continuous batching.
    - single worker / single thread (or CPU-only fallback) available even if slower.
    - record hardware/driver profile with runs so outputs can be reproduced.

### 3.3 Memory & Cognition

- The node should treat **long-term memory as a separate plane** with explicit lifecycle:
  - working/session context is transient,
  - episodic history is distilled over time,
  - semantic facts persist with decay and can be pinned,
  - procedural knowledge is versioned.
- The system must support **local-only long-term memory** by default; cloud/council access is limited to distilled facts or session context unless explicitly allowed.
- To avoid **cognitive offloading debt**, default UX for learning/creative flows should:
  - prompt user effort (outline/selection) before full generation,
  - attribute which parts were AI- vs user-authored,
  - expose which memories were retrieved and why (traceable recall).
- On lower-memory hardware (e.g. 64 GB unified), the system should automatically tighten context budgets, model sizes, and cache policies while preserving the above behaviours.

---

## 4. Cloud Integration Requirements

### 4.1 Controlled External Calls

- The system may use external AI services to:
  - handle tasks that are too difficult or large for local models,
  - obtain web search results or live data,
  - provide a second opinion on critical content.

- All such calls must be:
  - **Optional** – users can disable cloud usage entirely.
  - **Transparent** – it should be clear when an external service is used.
  - **Budgeted** – the system must enforce configurable ceilings on external usage.

### 4.2 No Hidden Data Exfiltration

- No user data or model outputs should be sent externally without:
  - clear policy,
  - explicit user consent, and
  - a record that can be inspected.

---

## 5. Mesh / Multi-Node Behaviour

### 5.1 Single Logical Cluster

- Multiple Goni nodes on a network should behave as one **logical system**:
  - A user can connect to any node and still access:
    - their models,
    - their data,
    - their conversation history.

- The cluster should be able to:
  - distribute workloads across nodes,
  - assign latency-sensitive tasks appropriately,
  - schedule longer-running or heavy jobs on less busy nodes.

### 5.2 Simple Node Lifecycle

- Joining a cluster:
  - A new node must be joinable via a simple setup flow (e.g. pasting a join token or scanning a code).
- Leaving or failing:
  - If a node is shut down or fails, critical user data should not be lost.
  - The system should handle loss of a node gracefully, reducing capacity but not breaking basic functionality.

---

## 6. Security & Privacy Requirements

### 6.1 Data Ownership

- All user data and derived artefacts (embeddings, model adapters, logs) stored locally should:
  - remain under the user’s control,
  - be exportable and backup-able in a documented way.

### 6.2 Access Control

- Goni must support:
  - at least one **administrator** role per node/cluster,
  - separate **user accounts** where appropriate (for shared households or teams),
  - secure authentication mechanisms (passwords, optional hardware keys, etc.).

### 6.3 Encryption and Secure Channels

- The system should:
  - offer **encrypted storage** for sensitive data (e.g. via disk encryption).
  - use **encrypted communication channels** for:
    - API calls,
    - mesh traffic between nodes,
    - remote access.

### 6.4 Observability Without Surveillance

- Logs should:
  - be sufficient to diagnose issues and track resource usage,
  - avoid storing unnecessary sensitive content in plain text.

---

## 7. User Experience & Management

### 7.1 Setup Experience

- On first power-up, the system should:
  - be discoverable from a nearby device without requiring a monitor and keyboard,
  - provide a short, guided setup in a browser or companion app,
  - configure:
    - administrator account,
    - network settings,
    - optional disk encryption,
    - optional remote access.

### 7.2 Everyday Interaction

- The primary interaction channels should include:
  - Web UI (chat, configuration, dashboards),
  - API for integrations,
  - Optional mobile and desktop clients.

- Users should be able to:
  - see what the system is currently doing (e.g. indexing, training, idle),
  - view basic resource usage (e.g. “node is under heavy load, expect slower replies”),
  - manage connected data sources (add/remove email accounts, storage locations, etc.).

### 7.3 Updates

- The system should:
  - support regular, safe updates of the software stack,
  - distinguish between:
    - security/bugfix updates,
    - major feature or model updates,
  - allow the user to choose update windows (e.g. “nightly”, “manual”).

---

## 8. Extensibility & Integrations

- Goni should expose a **stable API** so that:
  - third-party tools and scripts can call its AI capabilities,
  - power users can automate workflows (e.g. CI, document pipelines).

- The system should allow **adding new capabilities** over time:
  - new local models,
  - new connectors to data sources,
  - new external AI services,
  without requiring a full reinstall.

---

## 9. Explicit Non-Goals (Software MVP)

For the MVP, Goni software is **not** intended to be:

- A multi-tenant platform hosting unrelated users or organisations.
- A general-purpose container hosting platform for arbitrary workloads.
- A fully-fledged replacement for enterprise MLOps platforms.

These may be future directions, but the MVP focuses on **one owner (or small team) per Goni cluster**, with a strong emphasis on local-first personal AI.


