---
id: GONI-SPEC-ECC730993125
title: 7.1 Sovereign Mode (default)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Route default: DIRECT.'
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 7.1 Sovereign Mode (default)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 7.1 Sovereign Mode (default)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 7.1 Sovereign Mode (default)

- Route default: DIRECT.
- Overlay default: DENY unless explicitly allowed per purpose.
- Reliability-first; normal DNS/network stack.
- Receipts emitted with minimal metadata by default.
