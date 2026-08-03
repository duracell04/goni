---
id: GONI-IMAP-8DB21D221278
title: 5. MVP vs future cluster orchestrator
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**MVP** Single node only.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 5. MVP vs future cluster orchestrator
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 5. MVP vs future cluster orchestrator

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. MVP vs future cluster orchestrator

**MVP**

* Single node only.
* No cross-node routing.
* Simple mapping from HTTP request ? JobDescriptor.

**Future**

* Add a “target node” field on JobDescriptor and a Mesh layer that chooses nodes.
* Gateway orchestrator that can:

  * route interactive jobs to local node,
  * offload heavy jobs to remote nodes or cloud runtimes.

The Orchestrator spec here is the single-node kernel perspective; multi-node concerns live in mesh-and-wireguard.md.
