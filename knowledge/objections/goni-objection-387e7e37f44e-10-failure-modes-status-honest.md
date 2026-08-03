---
id: GONI-OBJECTION-387E7E37F44E
title: 10. Failure modes (status-honest)
type: objection
status: draft
implementation_state: not_applicable
proposition: 'Overlay unavailable: DENY (strict anonymity policy) PROMPT user to allow a one-off DIRECT route DEGRADE to Sovereign Mode only if policy allows Network Gate unavailable: fail closed; deny all egress and surface error.'
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
  heading: 10. Failure modes (status-honest)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 10. Failure modes (status-honest)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Failure modes (status-honest)

- Overlay unavailable:
  - DENY (strict anonymity policy)
  - PROMPT user to allow a one-off DIRECT route
  - DEGRADE to Sovereign Mode only if policy allows
- Network Gate unavailable: fail closed; deny all egress and surface error.
- Budget exhausted: deny and emit receipt with exhaustion flags.
- Council unreachable: fall back to local-only execution per
  blueprint/docs/remote-llm-architecture.md.
