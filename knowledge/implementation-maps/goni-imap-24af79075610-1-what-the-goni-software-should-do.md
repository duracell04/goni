---
id: GONI-IMAP-24AF79075610
title: 1. What the Goni software should do
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'At a high level, Goni software should enable the node to: Act as a **local AI assistant**: conversational chat interface, coding help, summarisation and explanation of documents and threads.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/00-overview.md
  heading: 1. What the Goni software should do
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 1. What the Goni software should do

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
