# Goni Software – Overview (MVP)

This folder covers the **software side of Goni**:

- how the node boots and runs its services,
- how it provides a local-first AI assistant,
- how it connects to user data sources and (optionally) cloud models,
- and how multiple nodes cooperate as a small cluster.

The goal is to define a software stack that turns the hardware into:

> a **personal AI appliance** – private by default, locally useful even offline, and able to scale out across several boxes.

We stay **technology-agnostic** here: no specific frameworks or vendors are mandated in this overview.  
Details about concrete tools and implementations belong in separate design documents.

---

## 1. What the Goni software should do

At a high level, Goni software should enable the node to:

- Act as a **local AI assistant**:
  - conversational chat interface,
  - coding help,
  - summarisation and explanation of documents and threads.

- Provide **retrieval-augmented generation (RAG)**:
  - ingest and index user-approved data sources (documents, notes, email, calendar, etc.),
  - answer questions using both models and the indexed data.

- Perform **light personalisation**:
  - allow training of small adapters or similar mechanisms on user data,
  - stay within the compute envelope of a small appliance (no full heavy fine-tunes on-device).

- Operate **local-first**:
  - run useful models and tools without an internet connection,
  - only call external AI services when explicitly allowed and beneficial.

- Expose a **clean interface**:
  - web UI for chat and configuration,
  - network API for editors, terminals, and other tools,
  - optional remote access from the user’s other devices.

- Participate in a **mesh / cluster**:
  - multiple Goni nodes on a network should behave as one logical system,
  - tasks can be spread across nodes while the user interacts with a single endpoint.

---

## 2. What this folder contains

- [`10-requirements.md`](./10-requirements.md)  
  The main reference for **software requirements** (capabilities, security, UX expectations, mesh behaviour, cloud usage policy).  
  Any architectural proposal should be checked against these requirements.

- `20-architecture.md`  
  (To be filled) High-level architecture for:
  - single-node service layout,
  - multi-node / mesh topology,
  - how local models, data indexing, orchestration, and APIs fit together.

- `30-components/`  
  Detailed notes for each major component:
  - base OS and provisioning,
  - orchestrator logic (routing between local and cloud),
  - model runtime, vector database, mesh coordination,
  - any background workers or agents.

- `40-apis-and-ui/`  
  Description of:
  - public API surface (e.g. chat, completion, tools),
  - authentication and access control,
  - dashboard / management UI concepts.

- `90-decisions.md`  
  Accepted software design decisions (ADR-style). Each entry should briefly describe the choice, alternatives, and rationale.

---

## 3. How to contribute (software)

1. Start with [`10-requirements.md`](./10-requirements.md) to understand what the software **must** achieve.
2. For new architectural ideas:
   - sketch them in `20-architecture.md` or a new document under `30-components/`,
   - open a software issue summarising the proposal and linking to the doc.
3. For API or UI changes:
   - update the relevant files under `40-apis-and-ui/`,
   - include example flows or payloads where helpful.
4. When a proposal is accepted:
   - add an entry to `90-decisions.md` to record the decision and its consequences.

The aim is to arrive at a **coherent, minimal software stack** that implements the agreed requirements and can evolve as hardware and AI tools advance.
