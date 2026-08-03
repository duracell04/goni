---
id: GONI-SPEC-C0EC9D0B0A06
title: 3. Interfaces (kernel API)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Minimal kernel APIs (names are illustrative): read_state(snapshot_id) -> StateSnapshot append_delta(delta: StateDelta) -> delta_id checkpoint(snapshot: StateSnapshot) -> snapshot_id summarize(range, policy) -> LatentSummary All interfaces are capability-mediated and produce audit records.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 3. Interfaces (kernel API)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 3. Interfaces (kernel API)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Interfaces (kernel API)

Minimal kernel APIs (names are illustrative):

- `read_state(snapshot_id) -> StateSnapshot`
- `append_delta(delta: StateDelta) -> delta_id`
- `checkpoint(snapshot: StateSnapshot) -> snapshot_id`
- `summarize(range, policy) -> LatentSummary`

All interfaces are capability-mediated and produce audit records.
