---
id: TOOL-SURFACE-01
title: Model-Facing Tool Surface
type: specification
status: draft
implementation_state: specified_only
proposition: For each inference step, Goni should compile the smallest sufficient model-visible tool surface from installed tool manifests, current policy and capability eligibility, WorkOrder relevance, and deployment-profile constraints; model visibility does not itself grant kernel execution authority.
domains:
- tools
- agent
- harness
- system
aliases:
- model tool view
- tool projection
relations:
- type: refines
  target: TOOL-01
- type: depends_on
  target: GONI-SPEC-F37FC6D98E05
- type: depends_on
  target: HARNESS-RUNTIME-01
- type: depends_on
  target: MODEL-DEPLOY-01
- type: refines
  target: INTERFACE-SELECTION-01
sources:
- SRC-PATIL2025-BFCL
artifacts: []
uncertainty: Optimal tool-surface size and selection policy are model- and workload-dependent. The contract specifies separation of concerns, not one universal routing algorithm.
legacy: []
---

# Model-Facing Tool Surface

Goni distinguishes the canonical tool capability model from the representation shown to a particular model invocation.

The kernel-facing contract remains `TOOL-01`. `TOOL-SURFACE-01` defines only how eligible capabilities are projected into the model's cognitive environment.

## Tool-set relation

For a given WorkOrder:

```text
T_model ⊆ T_eligible ⊆ T_installed
```

where:

- `T_installed` is the set of registered tool capabilities;
- `T_eligible` is the subset permitted by current policy, environment, and capability prerequisites; and
- `T_model` is the subset actually exposed to the selected model for the current inference step.

A tool can therefore be installed and even policy-eligible without consuming model context on every turn.

## Canonical ToolManifest

A canonical tool manifest should identify at least:

- `tool_id` and version;
- operations or action identifiers;
- bounded description of semantics;
- input and output schema refs;
- epistemic or causal effect classification where applicable;
- required capabilities;
- credential and egress requirements;
- sandbox profile;
- preconditions and postconditions;
- idempotency semantics;
- retry semantics;
- transaction or compensation support;
- verification contract;
- adapter or transport type; and
- provenance/version refs.

The manifest is Goni-owned metadata. Native provider function definitions, MCP descriptions, CLI help text, OpenAPI fragments, or vendor SDK metadata may inform an adapter but do not become the canonical authority record automatically.

## ModelToolView

For one inference episode, the harness compiles a `ModelToolView` containing:

- `work_order_id`;
- `tool_surface_id`;
- selected deployment-profile ref;
- visible tool and operation IDs;
- model-facing descriptions;
- schema refs or compiled schema representations;
- manifest hashes;
- selection basis;
- token or tool-count budget when relevant; and
- adapter/protocol information needed by the selected runtime.

The `ModelToolView` may be provider-specific in representation while preserving provider-independent tool semantics.

## Selection

Tool projection should consider:

- WorkOrder objective and planned next step;
- action versus observation needs;
- authority and policy eligibility;
- current state and environment;
- model tool-calling capability;
- semantic overlap among tools;
- tool-schema context cost;
- expected value of exposing an additional tool; and
- verification and rollback availability.

The default objective is the smallest sufficient tool surface, not the largest available catalogue.

## Boundary

A model-visible tool definition is not a capability token. It states that the model may propose an operation. Actual execution still requires the normal kernel path:

```text
ModelToolView
-> ToolProposal
-> structural / semantic validation
-> policy and capability mediation
-> TOOL-01 syscall
-> transaction / effect
-> receipt
```

This separation lets Goni change models, native tool formats, MCP adapters, structured-decoding engines, or tool descriptions without changing authority semantics.
