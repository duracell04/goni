---
id: GONI-IMAP-32706FDAC206
title: 7. Existing Goni mapping
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The runtime is a cross-contract posture over the existing architecture: | Existing responsibility | Local sovereign use | | Knowledge Plane | Preserve sources, derived memory, provenance, validity, permissions, and conflict state.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/65-local-sovereign-knowledge-runtime.md
  heading: 7. Existing Goni mapping
  revision: 43414875152ae18f9977f21c9786b2d7025081ac
---

# 7. Existing Goni mapping

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Existing Goni mapping

The runtime is a cross-contract posture over the existing architecture:

| Existing responsibility | Local sovereign use |
| --- | --- |
| Knowledge Plane | Preserve sources, derived memory, provenance, validity, permissions, and conflict state. |
| Context Plane | Materialize bounded, source-linked evidence without granting it authority. |
| Control Plane | Apply principal-owned policy, delegation, budgets, scheduling, and stopping decisions. |
| Execution substrate | Run replaceable local models and capability-scoped tools. |
| Harness | Keep prompts, retrieval, routing, proposals, and commits inspectable and separable. |
| Receipts | Reconstruct meaningful knowledge changes and mediated effects for the owner. |

The primary contract mappings are:

- [Governed Memory Retrieval](/blueprint/30-specs/memory-retrieval.md) for source
  and derived-artifact separation.
- [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md) for
  temporal, contradiction, identity, and ontology behavior.
- [Receipts](/blueprint/30-specs/receipts.md) for purpose-limited
  reconstruction.
- [Tool Capability API](/blueprint/30-specs/tool-capability-api.md) for the hard
  expression/effects boundary.
