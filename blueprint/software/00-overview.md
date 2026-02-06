# Goni Software - Overview (MVP)

This folder covers the **software side of Goni**:

- how the node boots and runs its services,
- how it provides a local-first AI assistant,
- how it connects to user data sources and (optionally) cloud models,
- and how multiple nodes cooperate as a small cluster.

The goal is to define a software stack that turns the hardware into a **personal AI appliance**: private by default, useful offline, and able to scale out across several boxes.

Goni separates **cognition** from **verbalization**. Most work happens in a latent state: embeddings, structured summaries, and compact "world state" variables that are continuously updated from observations (files, screen, messages, sensors, tool outputs). Natural language output is treated as an **optional projection** used only when useful (explain, draft, converse). This reduces unnecessary token generation, improves privacy boundaries (less raw text duplication), and makes the system easier to run locally by keeping always-on components lightweight.

**Agents are local processes; the LLM is a budgeted interrupt.** The canonical
specs live in `blueprint/30-specs/`:

- Latent state: `blueprint/30-specs/latent-state-contract.md`
- Agents and manifests: `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Tools and audit: `blueprint/30-specs/tool-capability-api.md`
- Scheduler/interrupts: `blueprint/30-specs/scheduler-and-interrupts.md`

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
  - optional remote access from the user's other devices.

- Participate in a **mesh / cluster**:
  - multiple Goni nodes on a network should behave as one logical system,
  - tasks can be spread across nodes while the user interacts with a single endpoint.

---

## 2. What this folder contains

- [`10-requirements.md`](/blueprint/software/10-requirements.md)
  The main reference for **software requirements** (capabilities, security, UX expectations, mesh behaviour, cloud usage policy). Any architectural proposal should be checked against these requirements.

- [`20-architecture.md`](/blueprint/software/20-architecture.md)
  (To be filled) High-level architecture for single-node service layout, multi-node / mesh topology, and how local models, data indexing, orchestration, and APIs fit together.

- `30-components/`
  Detailed notes for each major component: base OS and provisioning, orchestrator logic (routing between local and cloud), model runtime, vector database, mesh coordination, and any background workers or agents.
  - See [30-components/latent-predictor.md](/blueprint/software/30-components/latent-predictor.md) for the latent-first cognition pattern and its integration points.

- `40-apis-and-ui/`
  Description of the public API surface (e.g. chat, completion, tools), authentication and access control, and dashboard / management UI concepts.

- `50-data/`
  Data spine, planes, and TXT axiom. Start with [50-data/00-index.md](/blueprint/software/50-data/00-index.md), then [50-data/10-axioms-and-planes.md](/blueprint/software/50-data/10-axioms-and-planes.md), and [50-data/53-schema-dsl-and-macros.md](/blueprint/software/50-data/53-schema-dsl-and-macros.md) to see how the Arrow schema DSL maps into the kernel.

- [`90-decisions.md`](/blueprint/software/90-decisions.md)
  Accepted software design decisions (ADR-style). Each entry should briefly describe the choice, alternatives, and rationale.

Runnable references:
- local stack: goni-prototype-lab:deploy/docker-compose.yml
- k8s overlays: goni-prototype-lab:deploy/k8s/

---

## 3. How to contribute (software)

1. Start with [`10-requirements.md`](/blueprint/software/10-requirements.md) to understand what the software **must** achieve.
2. For new architectural ideas:
   - sketch them in [20-architecture.md](/blueprint/software/20-architecture.md) or a new document under `30-components/`,
   - open a software issue summarising the proposal and linking to the doc.
3. For API or UI changes:
   - update the relevant files under [40-apis-and-ui/](/blueprint/software/40-apis-and-ui),
   - include example flows or payloads where helpful.
4. When a proposal is accepted:
   - add an entry to [90-decisions.md](/blueprint/software/90-decisions.md) to record the decision and its consequences.

The aim is to arrive at a **coherent, minimal software stack** that implements the agreed requirements and can evolve as hardware and AI tools advance.

---

## Kernel crate map (MVP)

- `goni-core`: wires planes together; orchestrator surface used by HTTP and CLI fronts.
- `goni-store`: data plane abstraction; Arrow spine and Qdrant stub.
- `goni-context`: context selector and KV pager traits; TXT axiom-aware helpers.
- `goni-sched`: scheduler traits and in-memory scheduler.
- `goni-router`: routing and escalation policy decisions.
- `goni-infer`: inference engine abstraction and HTTP vLLM client.
- `goni-schema`: generated Arrow schemas for the planes (from `50-data` DSL).
- `goni-http` / `goni-cli`: thin entrypoints that exercise the kernel.

