---
id: GONI-EVIDENCE-C7E2E223D66C
title: 'Source claim: JustVugg Colibrì'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Colibrì presents an inference runtime in which model components are placed across heterogeneous memory tiers and promoted, cached, or prefetched according to routing demand.
domains:
- research
- architecture
aliases: []
relations:
- type: supports
  target: GONI-SPECIFICATION-C64D8275BA34
sources:
- SRC-JUSTVUGG2026-COLIBRI
artifacts: []
uncertainty: This is repository-level engineering evidence rather than peer-reviewed validation. Claimed performance and behavior should be independently reproduced before being treated as measured GONI evidence.
legacy: []
---

# Source claim: JustVugg Colibrì

## Reported observation

The Colibrì repository describes a local mixture-of-experts inference runtime that manages expert placement across faster and slower storage or memory tiers. Its design uses routing demand, caching, hot-state placement, migration, and prefetching so that frequently needed components can remain closer to execution while colder components remain recoverable from lower tiers.

## GONI interpretation

Colibrì is useful as an engineering analogy for cognitive residency: an item's placement can change access latency without redefining the item's identity. This supports GONI's separation of semantic memory class from residency and its preference for demotion over destructive deletion when recovery remains valuable.

## Limitation

Model-weight or expert placement is not equivalent to semantic memory management. Colibrì does not establish GONI's provenance, truth, retention, page-fault, or authority rules.
