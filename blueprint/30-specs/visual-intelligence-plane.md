---
id: VIS-01
type: SPEC
status: specified_only
---
# Visual Intelligence Plane
DOC-ID: VIS-01
Status: Specified only / roadmap

Goni treats visual work as governed visual action, not prompt-to-image
entertainment. Computer vision, image generation, image editing, visual memory,
and visual receipts are execution capabilities under the same Work Order, Done
Contract, capability policy, model registry, receipt, and rollback rules as the
rest of Goni.

The model makes pixels. The kernel makes those pixels accountable.

Core invariant: Goni owns authority; visual models and ComfyUI-style workflows
are replaceable execution substrates. Visual delegation is authority first,
execution second, receipt always. Goni MUST NOT become "ComfyUI with
governance"; it governs visual delegation and may use ComfyUI-compatible
workflows as one backend.

## 1. Scope

This spec applies to tasks that inspect, transform, generate, annotate, compare,
or remember image-like artifacts, including screenshots, diagrams, document
pages, product photos, brand assets, mockups, evidence images, masks, and
generated outputs.

Visual execution backends may include ComfyUI-compatible workflow engines,
diffusion runtimes, segmentation models, open-set detectors, OCR/layout
analyzers, visual-language models, and embedding models. These backends are
replaceable substrate. They do not own authority, approval corridors, asset
permissions, memory promotion, receipts, or rollback.

## 2. Canonical flow

Every meaningful visual task follows this logical flow:

```text
Visual Work Order
-> Visual Done Contract
-> source intake and asset permissions
-> computer-vision analysis
-> mask/object/layout extraction
-> generation or editing
-> post-generation verification
-> visual receipt
-> memory update or export
```

The flow may stop early when the requested task is analysis-only, audit-only,
or blocked by policy. A stopped flow still emits the required receipt for the
mediated decision.

## 3. Visual Work Order

A visual Work Order extends the DELEG-INT-01 WorkOrder with visual-specific
metadata stored as summaries and stable refs, not raw private content:

```yaml
visual_work_order:
  task_class: "visual.logo_refinement | visual.image_edit | visual.design_board | visual.diagram | visual.screenshot_audit | visual.product_visual | visual.evidence_annotation"
  source_assets: ["visual_asset_id", "brand_kit_id", "reference_id"]
  output_target: "one final version"
  constraints:
    style: "minimal | premium | technical | project-specific"
    preserve: ["logo geometry", "person identity", "brand colors"]
    avoid: ["fake text", "unlicensed brand imitation", "overstylized clutter"]
  approval_corridor: "no_go | soft_gate | autopilot | escalated"
  visual_asset_permission_class: "public_reference | project_owned | private_screenshot | person_identifying | brand_sensitive | legal_evidence"
```

The canonical WorkOrders table remains the storage anchor. Visual fields are
preserved through `input_refs`, `constraint_summary`, `tools`,
`output_schema_ref`, `risk_class`, and `provenance` until a future schema
version adds first-class visual Work Order columns.

## 4. Visual Done Contract

A visual Done Contract extends DELEG-INT-01 DoneContract with visual completion
criteria:

```yaml
visual_done_contract:
  deliverable: "one final image, annotated asset, visual audit, or design recommendation"
  must_include:
    - "clear visual hierarchy when design is requested"
    - "legible text when text is present"
    - "consistent style with project context"
  must_verify:
    - "no unwanted object changes"
    - "no private/contextual leakage"
    - "text and logo are readable"
    - "source/reference assets respected"
  stop_condition: "ready for user approval, memory update, or export"
```

Evidence and legal visual tasks MUST use `audit_grade` work-quality mode. Their
Done Contract must preserve evidence scope, source-faithfulness limits, and
negative-claim policy. Goni MUST NOT convert "not found in checked image set"
into "does not exist" without adequate scope.

## 5. Visual task profiles

Visual tasks are not interchangeable. The Work Order task class determines the
minimum asset permission intake, capability checks, verification scope, and
`visual_basis` receipt content.

| Profile | Task classes | Permission posture | Receipt and verification requirements |
| --- | --- | --- | --- |
| Visual analysis | `visual.diagram`, object/layout inspection, document-page understanding | May run on public/project assets under normal policy; private screenshots require minimization and local-first routing by default. | `visual_basis` records source asset refs/hashes, analyzer bundle, compact findings summary, verification limits, and memory refs if findings are retained. |
| Visual generation | `visual.design_board`, `visual.product_visual`, new diagram or mockup creation | Requires license-compatible model bundle, rights-compatible references, and brand/person checks when references are sensitive. | `visual_basis` records model bundle, manifest hash, workflow hash, prompt summary, references used, output hash, license/rights check, and approval/export state. |
| Visual editing/transformation | `visual.image_edit`, `visual.logo_refinement`, inpainting, outpainting, resize, upscale, style transfer | Requires source ownership or explicit permission; person-identifying, brand-sensitive, and private assets require stricter gates. | `visual_basis` records source and output hashes, mask/control refs, transformation summary, object-preservation check, rollback ref, and approval state. |
| Evidence annotation | `visual.evidence_annotation`, comparison, reversible markup | Always `audit_grade`; legal/evidence assets default to no destructive alteration and no generation that could be confused with evidence. | `visual_basis` records source hashes, annotation artifact refs, workflow hash, evidence scope, source-faithfulness check, chain/rollback refs, and negative-claim limits. |
| Screenshot/design audit | `visual.screenshot_audit`, UI critique, layout reading, accessibility/design audit | Private screenshots require redaction/minimization; audits may use analysis-only paths without generation. | `visual_basis` records screenshot hash, OCR/layout refs, audit criteria, issue summary refs, privacy leakage check, and no generated-output claim unless generation occurred. |

If a task combines profiles, the stricter permission posture and receipt
requirements apply. Evidence annotation must not be silently downgraded to
ordinary editing, and screenshot/design audit must not silently become
generation.

## 6. Asset permission classes

Visual tasks carry an asset permission class because images often contain
identity, private context, rights, or evidence value:

| Class | Default posture |
| --- | --- |
| `public_reference` | Low-risk analysis or generation reference when rights are known. |
| `project_owned` | Allowed for project workflows under project policy. |
| `private_screenshot` | Requires minimization, leakage checks, and local-first routing by default. |
| `person_identifying` | Requires stricter approval for identity-preserving edits, face use, or export. |
| `brand_sensitive` | Requires brand/right checks and imitation limits. |
| `legal_evidence` | Audit-grade only; transformations must be annotation, comparison, or reversible preparation unless explicitly approved. |

Policy may add narrower classes. More sensitive classes raise the approval
corridor, assurance floor, and receipt tier.

## 7. Execution substrate

Visual models enter through MODEL-REG-01 as governed model bundles. Examples of
substrate families include FLUX-style image generators, Qwen-Image-style
typography/editing models, Stable Diffusion-family workflows, SAM-style
segmentation, open-set detectors, Florence/Qwen-VL/InternVL-style visual
reasoning, and CLIP/OpenCLIP/DINO-style embeddings.

Workflow engines such as ComfyUI-compatible node graphs are execution backends,
not control planes. Goni compiles the visual Work Order into a workflow
template, supplies policy-approved model bundle IDs and asset refs, receives
output hashes and intermediate refs, performs verification, and emits the
canonical receipt.

Reference anchors: FLUX licensing and bundle variants [[bfl-flux-repo]],
Qwen-Image typography/editing direction [[qwen-image-2-2026]], SAM 2
segmentation [[sam2-2024]] [[meta-sam2-page]], and ComfyUI-style node workflows
[[comfyui-repo]].

## 8. Visual memory

Visual memory is governed asset metadata plus content-addressed artifacts. Raw
image binaries, masks, and large OCR text do not live in Control-plane rows.
They are addressed by hash and storage URI, with metadata stored in canonical
VisualAssets and VisualAssetDerivations rows.

Required logical metadata includes:

- `visual_asset_id`
- asset type (`logo | screenshot | product_photo | diagram | social_post | document_page | mask | generated_output`)
- source hash and storage URI
- rights status (`owned | licensed | public | unknown`)
- privacy class and permission scope
- project/person refs
- style tags
- detected object refs
- OCR chunk refs
- embedding refs
- derived-from refs
- approved-output refs
- receipt refs

Visual memory writes are governed memory writes. They must carry provenance,
permission scope, receipt refs, and review/expiry policy where applicable.

## 9. Visual receipts

VIS-01 extends REC-01 with a `visual_basis` object. It is not a separate
receipt type.

`visual_basis` SHOULD include:

- `source_asset_hashes`
- `source_asset_refs`
- `model_bundle_id`
- `manifest_hash`
- `workflow_hash`
- `prompt_summary`
- `mask_refs`
- `control_refs`
- `transformations`
- `verification`
- `output_hash`
- `rollback_ref`

`prompt_summary`, `transformations`, and `verification` must be compact
metadata. Receipts MUST NOT store raw private screenshots, full OCR text,
unbounded prompts, or raw image binaries by default.

## 10. Verification

Before presenting generated or edited visual work as final, Goni SHOULD evaluate
the output against the Done Contract using task-appropriate checks:

- text/OCR legibility
- layout hierarchy
- object preservation
- mask accuracy
- style consistency
- brand consistency
- source-faithfulness
- private-data leakage
- license and rights compatibility
- genericness or overstyle

Verification results are evidence for the receipt, not proof that the image is
safe or correct. Failures must either trigger retry, downgrade the result to a
draft/recommendation, request approval, or block.

## 11. Evaluation metrics

Visual evaluation SHOULD track:

- approval rate
- edit distance or regeneration count
- text/OCR accuracy
- object preservation score
- mask IoU where ground truth exists
- brand consistency score
- visual similarity to approved references
- privacy leakage rate
- license conflict rate
- latency and GPU cost

The benchmark is whether the visual output satisfies the Done Contract with
minimal correction and no governance violation.

## 12. Invariants

- Visual actions require a Work Order and Done Contract.
- Private, person-identifying, brand-sensitive, and legal/evidence assets
  require stricter gates than public reference assets.
- Generated or edited outputs require source hashes, workflow hashes, output
  hashes, and rollback refs when a prior version exists.
- Visual receipts must omit raw private content by default.
- Model bundles may only run visual task classes allowed by MODEL-REG-01.
- Workflow backends and third-party logs cannot replace canonical Goni
  receipts.
- Visual memory stores governed metadata and refs; raw binaries remain
  content-addressed artifacts outside Control-plane records.
- Mixed visual tasks inherit the strictest permission posture, receipt fields,
  and verification requirements among their task profiles.

## 13. Upstream

- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Model bundle registry governance](/blueprint/30-specs/model-registry.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)

## 14. Downstream

- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Visual runtime](/blueprint/software/30-components/visual-runtime.md)
- [Metrics](/blueprint/docs/metrics.md)

## 15. Adjacent

- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## Conformance tests

- visual actions require a Work Order and Done Contract
- private/person/evidence assets require stricter gates than public references
- generated or edited outputs require source hashes, workflow hashes, output
  hashes, and rollback refs
- visual receipts must omit raw private content by default
- model bundles may only run allowed visual task classes
- third-party workflow logs cannot replace Goni receipts
- legal/evidence visual tasks must run in audit-grade mode
- analysis, generation, editing/transformation, evidence annotation, and
  screenshot/design audit must preserve distinct permission and receipt
  requirements
- visual memory writes must preserve rights status, permission scope,
  provenance, and receipt refs
- verification failures must block, retry, request approval, or downgrade the
  deliverable rather than being silently ignored
