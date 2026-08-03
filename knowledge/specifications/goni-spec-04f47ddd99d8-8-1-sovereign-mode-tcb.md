---
id: GONI-SPEC-04F47DDD99D8
title: 8.1 Sovereign Mode TCB
type: specification
status: draft
implementation_state: specified_only
proposition: 'Trust required for policy enforcement and audit: Goni kernel policy engine and capability token store.'
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
  heading: 8.1 Sovereign Mode TCB
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 8.1 Sovereign Mode TCB

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 8.1 Sovereign Mode TCB

Trust required for policy enforcement and audit:

- Goni kernel policy engine and capability token store.
- Network Gate (PDP/PEP).
- Local OS networking stack and firewall rules that enforce default-deny.
- Audit log writer (Control plane).
