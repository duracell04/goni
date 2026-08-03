---
id: GONI-PRINCIPLE-56C483B17A9B
title: 4. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**Append-only deltas:** StateDeltas are never modified or deleted in place.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 4. Invariants
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 4. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Invariants

- **Append-only deltas:** `StateDeltas` are never modified or deleted in place.
- **Crash consistency:** state is reconstructable from the latest snapshot plus
  ordered deltas within the retention window.
- **Policy mediation:** every write is validated by the policy engine.
- **Latent-first loop:** steady-state updates do not require LLM decoding.
- **Provenance attached:** each record includes `provenance` metadata.
