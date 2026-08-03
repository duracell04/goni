---
id: GONI-SPEC-4EB1B8A4EFA4
title: 10. Financial And Commercial Linkage
type: specification
status: draft
implementation_state: specified_only
proposition: When a robot action searches for goods or services, negotiates, contracts, reserves budget, purchases, pays, accepts delivery terms, or creates a financially binding commitment, the action MUST also satisfy DAT-01.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 10. Financial And Commercial Linkage
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 10. Financial And Commercial Linkage

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Financial And Commercial Linkage

When a robot action searches for goods or services, negotiates, contracts,
reserves budget, purchases, pays, accepts delivery terms, or creates a
financially binding commitment, the action MUST also satisfy
[DAT-01](/blueprint/30-specs/delegated-agent-treasury.md).

Examples include:

- a home robot ordering supplies,
- a warehouse robot paying for an API or task-specific service,
- a service robot accepting delivery or repair terms,
- a robot booking external maintenance,
- a robot choosing between vendors for replacement parts.

The robot mandate does not create financial authority. Financial authority
requires a Delegated Agent Treasury, Negotiation Mandate, budget reservation,
approval thresholds, and settlement receipt path under DAT-01.
