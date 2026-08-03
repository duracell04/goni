---
id: NET-01
title: NET-01 - Network Gate and Anonymity
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: NET-01 Status: Specified only / roadmap Conformance: TBD (goni-lab harness) This spec defines network egress control for Goni OS.'
domains:
- network
- specs
aliases:
- NETWORK-GATE-AND-ANONYMITY
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: NET-01 - Network Gate and Anonymity
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# NET-01 - Network Gate and Anonymity

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# NET-01 - Network Gate and Anonymity
DOC-ID: NET-01
Status: Specified only / roadmap
Conformance: TBD (goni-lab harness)

This spec defines network egress control for Goni OS. Networking is treated as
a capability-scoped syscall mediated by a reference monitor (Network Gate).
It formalizes two policy bundles ("Sovereign Mode" and "Anonymous Mode"),
adversary models, audit receipts, and failure modes.
