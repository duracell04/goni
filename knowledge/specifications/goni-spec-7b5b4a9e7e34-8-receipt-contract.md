---
id: GONI-SPEC-7B5B4A9E7E34
title: 8. Receipt contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'Model execution receipts SHOULD include: provider model_id bundle_id manifest_hash assurance_level ml_bom_ref attestation_refs policy_hash adapter_refs when adapters are active prompt_policy_bundle_ref when prompt or policy bundle selection affected output or eligibility memory_bundle_refs when memory or retrieval bundles affected output or'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 8. Receipt contract
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 8. Receipt contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Receipt contract

Model execution receipts SHOULD include:

- `provider`
- `model_id`
- `bundle_id`
- `manifest_hash`
- `assurance_level`
- `ml_bom_ref`
- `attestation_refs`
- `policy_hash`
- `adapter_refs` when adapters are active
- `prompt_policy_bundle_ref` when prompt or policy bundle selection affected
  output or eligibility
- `memory_bundle_refs` when memory or retrieval bundles affected output or
  eligibility
- `eval_receipt_refs` when the route depends on prior evaluation.

Bundle promotion, adapter promotion, model-stack activation, rollback,
deletion, and policy override are mediated actions and MUST emit receipts.

Model hubs such as Hugging Face, ModelScope, or any other registry are
discovery sources, not trust boundaries. Goni decides locally whether a model
may run, access private memory, call tools, or operate in sensitive contexts.

Visual model execution receipts SHOULD also include `modality`,
`visual_capabilities`, `workflow_runtime`, `allowed_asset_classes`, and local
visual eval receipt refs when those fields affected eligibility. See VIS-01 for
the visual receipt extension.

Visual bundle metadata is grounded in primary substrate references rather than
brand shorthand: FLUX license/state differences [[bfl-flux-repo]],
Qwen-Image text/editing capability direction [[qwen-image-2-2026]], and
ComfyUI-style workflow runtime provenance [[comfyui-repo]].
