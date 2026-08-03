---
id: GONI-SPEC-619AE87186CB
title: 4. Capability tokens
type: specification
status: draft
implementation_state: specified_only
proposition: 'Capability tokens bind: allowed tool IDs, resource scopes (filesystem roots, network domains), budgets (tokens, CPU time, disk writes), expiry and revocation rules.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 4. Capability tokens
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 4. Capability tokens

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Capability tokens

Capability tokens bind:

- allowed tool IDs,
- resource scopes (filesystem roots, network domains),
- budgets (tokens, CPU time, disk writes),
- expiry and revocation rules.

Tokens are immutable and referenced by ID in tool calls.
