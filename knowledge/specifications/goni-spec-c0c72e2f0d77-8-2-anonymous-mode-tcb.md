---
id: GONI-SPEC-C0C72E2F0D77
title: 8.2 Anonymous Mode TCB
type: specification
status: draft
implementation_state: specified_only
proposition: 'All Sovereign TCB components, plus: Overlay Capsule (overlay client + dependencies).'
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
  heading: 8.2 Anonymous Mode TCB
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 8.2 Anonymous Mode TCB

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 8.2 Anonymous Mode TCB

All Sovereign TCB components, plus:

- Overlay Capsule (overlay client + dependencies).
- Gate-to-capsule proxy interface.

Anonymous Mode does not expand trust in the rest of the runtime; anonymity is
opt-in and compartmentalized.
