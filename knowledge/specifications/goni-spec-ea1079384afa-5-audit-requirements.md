---
id: GONI-SPEC-EA1079384AFA
title: 5. Audit requirements
type: specification
status: draft
implementation_state: specified_only
proposition: 'All LSS writes MUST include the following audit fields (directly or by reference): agent_id policy_hash state_snapshot_id provenance See blueprint/30-specs/tool-capability-api.md for the audit record envelope.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 5. Audit requirements
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 5. Audit requirements

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Audit requirements

All LSS writes MUST include the following audit fields (directly or by
reference):

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

See `blueprint/30-specs/tool-capability-api.md` for the audit record envelope.
