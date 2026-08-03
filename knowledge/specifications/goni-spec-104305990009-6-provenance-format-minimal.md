---
id: GONI-SPEC-104305990009
title: 6. Provenance format (minimal)
type: specification
status: draft
implementation_state: specified_only
proposition: 'provenance is a structured object that includes: source: origin (observer, encoder, tool, agent).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 6. Provenance format (minimal)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 6. Provenance format (minimal)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Provenance format (minimal)

`provenance` is a structured object that includes:

- `source`: origin (observer, encoder, tool, agent).
- `timestamp`: event time (UTC).
- `inputs`: references to upstream record IDs.
- `permissions`: policy tags in effect.
