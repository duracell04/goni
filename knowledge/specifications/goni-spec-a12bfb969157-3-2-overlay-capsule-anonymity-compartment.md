---
id: GONI-SPEC-A12BFB969157
title: 3.2 Overlay Capsule (anonymity compartment)
type: specification
status: draft
implementation_state: specified_only
proposition: The Overlay Capsule is a separate trust domain that hosts anonymity machinery and exposes a single local proxy endpoint to the Gate.
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
  heading: 3.2 Overlay Capsule (anonymity compartment)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 3.2 Overlay Capsule (anonymity compartment)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Overlay Capsule (anonymity compartment)

The Overlay Capsule is a separate trust domain that hosts anonymity machinery
and exposes a single local proxy endpoint to the Gate. It is opt-in and
isolated from the rest of the runtime to keep the Trusted Computing Base (TCB)
small for Anonymous Mode.
