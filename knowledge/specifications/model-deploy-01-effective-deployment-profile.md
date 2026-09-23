---
id: MODEL-DEPLOY-01
title: Effective Model Deployment Profile
type: specification
status: draft
implementation_state: specified_only
proposition: Goni should evaluate and route the exact deployed inference configuration rather than an abstract model family or checkpoint name because runtime, quantization, templates, context policy, decoding, tool adapters, cache policy, and hardware can materially alter observed behavior.
domains:
- models
- inference
- evaluation
- local-ai
aliases:
- deployment profile
- effective model configuration
relations:
- type: refines
  target: MODEL-REG-01
- type: depends_on
  target: GONI-PRINCIPLE-SYS-CAP-01
- type: depends_on
  target: GONI-PRINCIPLE-MODEL-ADAPTER-01
sources: []
artifacts: []
uncertainty: The relative contribution of each deployment dimension is workload-dependent. A profile records the tested system configuration; it does not imply that every field independently changes behavior materially.
legacy: []
---

# Effective Model Deployment Profile

`MODEL-REG-01` governs the provenance and approval of model bundles. `MODEL-DEPLOY-01` governs the concrete inference configuration actually evaluated and routed at runtime.

A model name alone is not a sufficient capability identifier.

A useful abstraction is:

```text
M_effective =
(weights,
 quantization,
 runtime,
 runtime_version,
 tokenizer,
 chat_template,
 context_policy,
 decoding,
 structured_output_backend,
 tool_adapter,
 cache_policy,
 hardware)
```

## DeploymentProfile

A deployment profile should identify at least:

- `deployment_profile_id`;
- approved `bundle_id`;
- runtime and exact version/revision;
- hardware profile or execution substrate;
- quantization/precision and relevant adapter state;
- tokenizer hash or pinned revision;
- chat/system template hash;
- maximum supported context and normal operating context policy;
- sampling and decoding configuration;
- reasoning mode/budget where exposed;
- structured-output or grammar backend;
- tool-call adapter/parser;
- KV/cache policy where configurable;
- local/remote execution class;
- concurrency or batching assumptions where relevant;
- evaluation receipt refs;
- observed resource profile refs; and
- creation/update timestamp.

## Evaluation identity

Capability claims should bind to:

```text
bundle_id + deployment_profile_id + eval_pack_id
```

rather than to a marketing model name alone.

For example, two deployments of the same checkpoint using different quantization, templates, runtimes, or tool-call adapters may receive separate evaluation receipts.

## Routing

The model router may use deployment-profile evidence for:

- instruction reliability;
- tool selection and argument quality;
- structured-output reliability;
- context-length behavior;
- retrieval-grounded task quality;
- latency and throughput;
- peak RAM/VRAM/unified-memory demand;
- energy or thermal behavior where measured;
- privacy/egress class; and
- task-specific success.

A stronger model profile may justify broader solution-space freedom inside an existing authority corridor. It does not grant additional authority.

## Change control

Changes to any field that can plausibly affect model behavior should trigger revalidation appropriate to the affected task classes. Harness mechanisms introduced to compensate for a prior deployment weakness should be reconsidered when the deployment profile changes materially.
