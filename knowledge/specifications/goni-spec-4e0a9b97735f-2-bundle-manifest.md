---
id: GONI-SPEC-4E0A9B97735F
title: 2. Bundle manifest
type: specification
status: draft
implementation_state: specified_only
proposition: 'Each approved bundle MUST have a manifest with: bundle_id upstream registry or source upstream model id and revision model card URI datasheet URI when dataset lineage is known ML-BOM or SBOM URI when available license id or license URI publisher or maintainer weight file hashes manifest hash quantization or precision runtime compatibility (llama.cpp, Ollama, vLLM, SGLang, or other)'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 2. Bundle manifest
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 2. Bundle manifest

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Bundle manifest

Each approved bundle MUST have a manifest with:

- `bundle_id`
- upstream registry or source
- upstream model id and revision
- model card URI
- datasheet URI when dataset lineage is known
- ML-BOM or SBOM URI when available
- license id or license URI
- publisher or maintainer
- weight file hashes
- manifest hash
- quantization or precision
- runtime compatibility (`llama.cpp`, `Ollama`, `vLLM`, `SGLang`, or other)
- approved task classes
- modality (`text | audio | image | video | multimodal`)
- visual capabilities when modality includes image or video (`generate`,
  `inpaint`, `outpaint`, `mask`, `segment`, `detect`, `ocr`, `layout`,
  `typography`, `style_transfer`, `upscale`, `embed`, `visual_qa`)
- visual workflow runtime when applicable (`diffusers`, `ComfyUI`, dedicated
  runtime, or other)
- visual allowed asset classes (`public_reference`, `project_owned`,
  `private_screenshot`, `person_identifying`, `brand_sensitive`,
  `legal_evidence`)
- private-memory permission (`deny | distilled_only | allowed_by_policy`)
- license state (`commercial_ok | noncommercial | research_only | unknown`)
- assurance level (`L0 | L1 | L2 | L3 | L4`)
- eval receipt refs
- local eval receipt refs when bundle eligibility depends on local visual evals
- attestation refs
- policy hash
- policy pack refs
- created-at timestamp

The manifest MUST be hash-addressed. A runtime MUST reject undeclared or
mutated bundle contents.

Conceptual artifacts:

- `ModelManifest`: the hash-addressed bundle manifest above. It is the local
  statement of what was downloaded, where it came from, what hashes matched,
  what license and provenance are known, and what task classes are allowed.
- `InstallReceipt`: the mediated receipt emitted when a model bundle is added,
  updated, quarantined, deleted, or made available to a runtime. It records
  source, hashes, manifest hash, installer identity, sandbox/runtime target,
  and policy result.
- `EvalReceipt`: the mediated receipt emitted by an evaluation run. It records
  bundle ID, manifest hash, eval pack, environment, dataset refs, result
  summary, failure disclosures, and limits of inference.
- `RollbackRef`: the stable reference to the prior approved bundle, policy
  state, runtime config, and cache/index state needed to reverse a promotion or
  quarantine a bad bundle.
- `AdapterManifest`: the hash-addressed statement for a LoRA, QLoRA,
  DPO-style, sparse expert, worldview lens, critic lens, or task adapter. It
  records base compatibility, adapter hashes, intended task classes, expected
  behavior change summary, license/provenance refs, eval refs, and policy hash.
- `TrainingDatasetLedgerRef`: the receipt-linked ref set for preference,
  correction, or training examples used to produce an adapter or promoted
  prompt/policy bundle. It stores dataset hashes and provenance summaries, not
  raw private examples by default.
- `AdaptationReceipt`: the mediated receipt emitted when an adapter, prompt
  bundle, policy bundle, memory bundle, or model stack is trained, evaluated,
  promoted, activated, deactivated, or rolled back.
- `AdapterRollbackRef`: the stable reference to the prior adapter set, prompt
  bundle, policy hash, eval state, and runtime config needed to disable or
  revert an adaptation.

These names are governance concepts. They may be stored as receipt fields,
manifest fields, or separate schema rows in a later version, but the logical
chain MUST exist before a bundle is promoted inward.
