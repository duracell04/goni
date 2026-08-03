---
id: GONI-IMAP-D5B9FDF5627B
title: Goni Software - Overview (MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'This folder covers the **software side of Goni**: how the node boots and runs its services, how it provides a local-first AI assistant, how it connects to user data sources and (optionally) cloud models, and how multiple nodes cooperate as a small cluster.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/00-overview.md
  heading: Goni Software - Overview (MVP)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Goni Software - Overview (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
