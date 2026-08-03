---
id: GONI-SYNTHESIS-B52DA93E132F
title: System map sketch (doc graph)
type: synthesis
status: draft
implementation_state: specified_only
proposition: System map sketch (doc graph)
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/00-system-map.md
  heading: System map sketch (doc graph)
  revision: be9473da26620477266f2911324846de58536b0b
---

# System map sketch (doc graph)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## System map sketch (doc graph)
```mermaid
graph TD
  A[Product Surfaces] --> B[Planes]
  B --> C[Subsystem Packets]
  C --> D[Contracts / Specs]
  D --> E[Governance: Privacy / Threat / TCB]
  C --> F[Evidence / Evaluation]
  A --> E
```
