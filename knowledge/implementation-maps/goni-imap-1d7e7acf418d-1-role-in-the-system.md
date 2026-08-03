---
id: GONI-IMAP-1D7E7ACF418D
title: 1. Role in the system
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The **Orchestrator** is the front door and job submission layer: Accepts external requests (HTTP, CLI, IDE), Validates and normalises them, Builds **job descriptors**, Hands them to the Control Plane (??) for scheduling, Streams results back to the client.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 1. Role in the system
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 1. Role in the system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role in the system

The **Orchestrator** is the front door and job submission layer:

- Accepts external requests (HTTP, CLI, IDE),
- Validates and normalises them,
- Builds **job descriptors**,
- Hands them to the Control Plane (??) for scheduling,
- Streams results back to the client.

It glues external APIs to the internal planes ?? (scheduler/router), ?? (context), and ?? (LLM runtime), without implementing their logic.

---
