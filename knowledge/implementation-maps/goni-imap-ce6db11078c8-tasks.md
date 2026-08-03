---
id: GONI-IMAP-CE6DB11078C8
title: Tasks
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: task_id = row_id Fields: request_id: fixed_size_binary[16], task_type: dict<uint8, utf8>, state: dict<uint8, utf8>, queue_id: dict<uint8, utf8>, expected_cost_tokens: uint32 Notes: Lyapunov inputs; append-only state transitions.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: Tasks
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# Tasks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Tasks
- PK: `task_id = row_id`
- Fields: `request_id: fixed_size_binary[16]`, `task_type: dict<uint8, utf8>`, `state: dict<uint8, utf8>`, `queue_id: dict<uint8, utf8>`, `expected_cost_tokens: uint32`
- Notes: Lyapunov inputs; append-only state transitions.
