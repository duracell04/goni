---
id: GONI-IMAP-0377E0E2A86D
title: Kernel crate map (MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'goni-core: wires planes together; orchestrator surface used by HTTP and CLI fronts.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/00-overview.md
  heading: Kernel crate map (MVP)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Kernel crate map (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Kernel crate map (MVP)

- `goni-core`: wires planes together; orchestrator surface used by HTTP and CLI fronts.
- `goni-store`: data plane abstraction; Arrow spine and Qdrant stub.
- `goni-context`: context selector and KV pager traits; TXT axiom-aware helpers.
- `goni-sched`: scheduler traits and in-memory scheduler.
- `goni-router`: routing and escalation policy decisions.
- `goni-infer`: inference engine abstraction and HTTP vLLM client.
- `goni-schema`: generated Arrow schemas for the planes (from `50-data` DSL).
- `goni-http` / `goni-cli`: thin entrypoints that exercise the kernel.
