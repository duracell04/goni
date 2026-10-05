---
id: GONI-SYNTHESIS-HARDWARE-COMPUTE-DEPENDENCIES
title: Physical dependencies of cooperative compute
type: synthesis
status: draft
implementation_state: specified_only
proposition: Heterogeneous compute placement connects to a branching hardware dependency
  model and the existing dated supplier map through measured backend readiness.
domains:
- hardware
- supply-chain
aliases: []
relations:
- type: synthesizes
  target: HCOMP-01
- type: synthesizes
  target: COMP-REG-01
- type: synthesizes
  target: GONI-IMAP-C4D26460E574
- type: synthesizes
  target: GONI-IMAP-FBBFE14FE1A3
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: The dependency diagram is a conceptual synthesis; actual supplier dependencies
  and backend readiness require dated source and measurement records.
legacy: []
---

# Physical dependencies of cooperative compute

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

This reading guide links [HCOMP-01](../principles/hcomp-01-semantic-universality-physical-specialization.md),
[COMP-REG-01](../specifications/comp-reg-01-compute-capability-registry.md), the
[existing hardware requirements](../specifications/goni-spec-5819106486b4-3-1-compute-capability.md),
and the [dated supplier map](../implementation-maps/goni-imap-c4d26460e574-25-hardware-layers-and-supplier-map-2025-2026.md).

## Dependency model

The proposal's EDA → Wafer → FabTools → Foundry → Packaging/HBM → System shorthand
is interpreted as a dependency graph, rather than a single production sequence:

```mermaid
flowchart TD
  EDA["EDA and IP"] --> Design["Chip design"]
  Design --> Fab["Foundry fabrication"]
  Materials["Wafers and materials"] --> Fab
  Tools["Fabrication tools"] --> Fab
  Fab --> Package["Packaging and integration"]
  Memory["Memory and HBM"] --> Package
  Package --> System["Compute system"]
  Memory --> System
  Power["Power and cooling"] --> System
  System --> Runtime["Supported runtime profile"]
```

This is an explanatory dependency model. It makes no new supplier, availability,
or market-share claims. Vendor facts remain dated and sourced in implementation
maps. A chip's existence, a purchasable system, a supported runtime, and a measured
GONI workload are separate readiness milestones.

The memory/interconnect → useful compute shorthand describes a workload
constraint, not a universal causal ordering. COMP-REG-01 and the existing
telemetry contract provide the placement-facing descriptions; empirical
performance and evidence costs belong to scoped evaluation results.
