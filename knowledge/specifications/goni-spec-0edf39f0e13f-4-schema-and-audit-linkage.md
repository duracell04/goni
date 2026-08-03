---
id: GONI-SPEC-0EDF39F0E13F
title: 4. Schema and audit linkage
type: specification
status: draft
implementation_state: specified_only
proposition: The canonical record is AgentManifests (see blueprint/software/50-data/51-schemas-mvp.md for the storage schema).
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-manifest.md
  heading: 4. Schema and audit linkage
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 4. Schema and audit linkage

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Schema and audit linkage

The canonical record is `AgentManifests` (see
`blueprint/software/50-data/51-schemas-mvp.md` for the storage schema). The following
audit fields are required:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`
