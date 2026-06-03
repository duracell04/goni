---
id: MODEL-REG-01
type: SPEC
status: specified_only
---
# Model Bundle Registry Governance
DOC-ID: MODEL-REG-01

Status: Specified only / roadmap

Open-weight release decentralizes access to model parameters, but not
necessarily governance over discovery, metadata, provenance, evaluation,
licensing, deployment, or runtime permissions. Goni may discover models through
public ecosystems, but approved execution flows through a governed bundle
registry. The runtime executes immutable bundle IDs whose provenance, license,
hashes, task permissions, assurance level, and evaluation receipts are known
before use. For personalized behavior, the execution unit may be a governed
model stack: base bundle plus approved adapters, prompt/policy bundle, and
memory or retrieval bundle refs.

Scientific framing:
- Observed fact: widely available model weights can broaden participation and
  enable local inference, while model openness still depends on documentation,
  code, data, licenses, and access structure. [[ntia2024-open-model-weights]]
- Theoretical inference: model hubs are governance infrastructure, not only file
  storage. They shape discovery, naming, metadata conventions, reputation,
  access restrictions, and takedown paths.
- Goni hypothesis: the unit of trust in open AI should shift from the hosted
  model repository to the locally attested model installation.

## 1. Scope

This spec applies to model bundles and governed model stacks used by local or
remote inference runtimes. It covers model provenance and approval metadata,
including adapter governance, but not the mechanics of training or runtime
scheduling.

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

## 3. Governed model stacks and adapters

Goni does not treat personalization as hidden drift inside the model. Prompt
steering, memory/retrieval, adapters, and full fine-tuning are separate
governed layers with different reversibility and review requirements.

A governed model stack is:

```text
base model bundle
+ optional adapter set
+ prompt/policy bundle
+ memory or retrieval bundle refs
+ runtime config
```

Adapters may encode domain skill, writing style, user worldview, neutral
academic framing, skeptical critic behavior, legal caution, sales style, or
other lenses. Plural adapters are preferred over one implicit ideology: a route
may compare user-worldview, neutral, critic, or specialist outputs when policy
or task risk requires it.

LoRA, QLoRA, DPO-style preference adapters, sparse expert modules, and similar
artifacts MUST be versioned as governed artifacts. A runtime MUST NOT silently
load an undeclared adapter or mutate model behavior without an
AdaptationReceipt. Route receipts MUST show which base bundle, adapters,
prompt/policy bundle, memory/retrieval refs, and eval refs were active when
those choices affected output or tool eligibility.

## 4. Registry roles

Goni distinguishes three roles:

- Public discovery: broad ecosystem search and metadata lookup.
- Private registry: local or self-hosted cache of approved bundles.
- Runtime loader: engine-specific loading from approved bundle IDs only.

Public discovery may include Hugging Face or ModelScope. Private registry
candidates may include self-hosted registries such as MatrixHub or KohakuHub
when they satisfy Goni policy, storage, and audit requirements. Runtime loading
may use engines such as Ollama, llama.cpp, vLLM, or SGLang.

Public hubs can decentralize access while centralizing epistemic mediation. In
Goni, the hub is a discovery input; the local bundle registry is the execution
authority.

## 5. Assurance levels

Goni uses graded assurance, not a trusted/untrusted binary:

| Level | Evidence | Maximum default use |
|-------|----------|---------------------|
| L0 | Hash only | Sandbox testing |
| L1 | Hash + manifest + license state | Public or low-risk tasks |
| L2 | Local eval receipt | Personal low-sensitivity memory |
| L3 | Signed third-party or community eval | Broader tool use |
| L4 | Reproducible provenance + ML-BOM + audit trail | Sensitive memory or enterprise use |

Assurance levels are permission ceilings, not guarantees. Policy may further
restrict a bundle below its assurance level.

## 6. Policy gates

Before a bundle or governed model stack may process private memory, policy MUST
check:

- license compatibility,
- source and publisher trust,
- hash match,
- assurance level,
- eval receipt coverage for the requested task class,
- adapter compatibility, adapter hashes, and adapter eval receipt coverage when
  adapters are active,
- prompt/policy bundle provenance and rollback state,
- memory or retrieval bundle refs and retention policy,
- private-memory permission,
- visual capability coverage and allowed asset class when the task is visual,
- visual workflow runtime provenance when a node graph or pipeline is used,
- network and retention policy for the runtime destination,
- policy pack provenance and override rules.

If any gate fails, the router MUST choose a safer approved bundle or block the
request.

Policy sources MUST be transparent, inspectable, and provenance-bearing. Goni
MUST support user-editable policies, signed policy packs, community or
enterprise overlays, and override receipts. Otherwise the local registry would
replace one hidden governance layer with another.

## 7. Evaluation limits

Local evaluation receipts are attestations, not proofs. They can show which
tests ran, under which environment, against which model hash, with which
results. For adapters, they can show bounded before/after behavior under a
named eval pack. They do not prove absence of backdoors, lawful training data,
future safety, semantic equivalence to upstream claims, or universal alignment
with a user's intent.

Distributed trust remains a separate problem: one Goni node SHOULD NOT accept
another node's eval receipt without signature validation, evaluator identity,
environment disclosure, failure disclosure, and policy-approved reputation or
attestation rules. SLSA, in-toto, SPDX, and CycloneDX are relevant source
patterns for this supply-chain evidence model. [[slsa-framework]]
[[in-toto-framework]] [[spdx-overview]] [[cyclonedx-mlbom]]

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

## 9. Upstream

- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Visual runtime](/blueprint/software/30-components/visual-runtime.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Visual Intelligence Plane](/blueprint/30-specs/visual-intelligence-plane.md)

## 10. Downstream

- [Local models supplier card](/blueprint/60-market/suppliers/local-models.md)
- [Goni Lab](/blueprint/docs/goni-lab.md)

## Conformance tests

- Runtime rejects a bundle with a mismatched manifest hash.
- Runtime rejects private-memory use when permission is `deny`.
- Runtime rejects sensitive-memory use below the configured assurance floor.
- Bundle installation emits an InstallReceipt with bundle and manifest hashes.
- Bundle promotion emits an EvalReceipt reference and RollbackRef.
- Adapter promotion emits an AdaptationReceipt with base bundle refs, adapter
  hashes, dataset ledger refs, eval refs, approval state, and AdapterRollbackRef.
- Runtime rejects undeclared adapters or model stacks with mismatched adapter
  hashes.
- Route selection can explain which base bundle, adapters, prompt/policy bundle,
  memory refs, and eval refs made the stack eligible for the task class.
- Route selection can explain why a bundle was eligible for the task class.
- Local eval receipts state test environment, model hash, result summary, and
  limits of inference.
- Visual routes reject bundles without required visual capabilities, allowed
  asset class, license state, workflow runtime provenance, and visual eval
  coverage for the requested task class.
