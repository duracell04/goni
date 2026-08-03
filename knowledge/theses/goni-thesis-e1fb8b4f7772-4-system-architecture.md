---
id: GONI-THESIS-E1FB8B4F7772
title: 4. System Architecture
type: thesis
status: draft
implementation_state: specified_only
proposition: 'Goni''s technical architecture is organized around a four-plane model: In formal notation, the node is represented in the software architecture as: where A is the Arrow Spine or canonical data substrate, X is the context-selection plane, K is the control plane for policy, scheduling, routing, and mediation, and E is the execution substrate for models, tools,'
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: 4. System Architecture
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 4. System Architecture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. System Architecture

Goni's technical architecture is organized around a four-plane model:

```text
Data Plane -> Context Plane -> Control Plane -> Execution Plane
```

In formal notation, the node is represented in the software architecture as:

```text
N = (A, X, K, E)
```

where `A` is the Arrow Spine or canonical data substrate, `X` is the
context-selection plane, `K` is the control plane for policy, scheduling,
routing, and mediation, and `E` is the execution substrate for models, tools,
sandboxes, and external effects. The detailed architecture is described in
[software/20-architecture.md](/blueprint/software/20-architecture.md).

This separation is intended to prevent a common failure mode in agent systems:
everything becoming prompt glue. In Goni, memory is not prompt history; tool
access is not ambient permission; network access is not implicit; logs are not
receipts; model output is not authority; and autonomy is not a personality trait
of the model. Each function belongs to a distinct system layer.

The architectural value of this model is that it makes personal AI governable.
Raw connectors are not intended to call models directly as an authority path.
Models are not intended to write canonical memory without mediation. Tools do
not execute with broad ambient authority in the target architecture. External
frameworks may be replaceable implementation substrates, but they do not become
the canonical source of
truth for memory, permission, receipts, policy, approval, or rollback.

This principle can be called sovereign modularity: components can be swapped,
but authority remains Goni-owned.
