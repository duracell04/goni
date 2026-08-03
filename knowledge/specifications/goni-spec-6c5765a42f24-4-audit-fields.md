---
id: GONI-SPEC-6C5765A42F24
title: 4. Audit fields
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every agent action that causes side effects must be attributable by: agent_id policy_hash state_snapshot_id provenance These fields are required on tool calls, state deltas, and commits.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-definition.md
  heading: 4. Audit fields
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 4. Audit fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Audit fields

Every agent action that causes side effects must be attributable by:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

These fields are required on tool calls, state deltas, and commits.
