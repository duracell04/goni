# Visual Runtime
DOC-ID: VIS-RUNTIME-01
Status: Specified only / roadmap

The Visual Runtime is the Execution Plane component that runs governed visual
analysis, generation, editing, and verification jobs for VIS-01. It is a
backend abstraction around replaceable visual engines, not the authority layer.
It exists to execute governed visual Work Orders, not to define whether they are
allowed.

## 1. Role in the system

The Visual Runtime:

- executes policy-approved visual workflow templates,
- loads only approved visual model bundle IDs,
- accepts source assets, masks, controls, and references by stable refs,
- returns output hashes and intermediate artifact refs,
- reports runtime capabilities and utilization to scheduling policy,
- exposes enough workflow provenance for receipts and rollback.

The Goni kernel owns Work Orders, Done Contracts, asset permissions,
capability tokens, model eligibility, receipts, memory updates, and approval
corridors. The runtime receives authority decisions; it does not create them.

## 2. Backend substrate

Visual execution may use ComfyUI-compatible node graphs, diffusion pipelines,
segmentation services, OCR/layout analyzers, open-set detectors, visual-language
models, or embedding extractors. These backends are adapters behind the Visual
Runtime interface.

ComfyUI-style graphs are useful because they make visual work explicit as
workflow nodes: load model, load reference, segment, mask, inpaint, control,
sample, upscale, verify, and save. Goni treats those graphs as hashed execution
templates. A graph hash is receipt evidence, not policy authority
[[comfyui-repo]].

## 3. Logical interface

```rust
pub struct VisualRequest {
    pub work_order_id: WorkOrderId,
    pub done_contract_hash: Hash32,
    pub task_class: VisualTaskClass,
    pub bundle_ids: Vec<BundleId>,
    pub workflow_hash: Hash32,
    pub source_asset_refs: Vec<VisualAssetRef>,
    pub mask_refs: Vec<ArtifactRef>,
    pub control_refs: Vec<ArtifactRef>,
    pub output_target: OutputTarget,
    pub deterministic_seed: Option<u64>,
}

pub struct VisualResult {
    pub status: VisualStatus,
    pub output_refs: Vec<ArtifactRef>,
    pub output_hashes: Vec<Hash32>,
    pub intermediate_refs: Vec<ArtifactRef>,
    pub verification_summary: VisualVerificationSummary,
    pub workflow_hash: Hash32,
    pub rollback_ref: Option<RollbackRef>,
}
```

The concrete API may differ, but implementations must preserve the logical
fields needed by VIS-01 and REC-01.

## 4. Responsibilities

- Validate that requested workflow templates are declared and hash-addressed.
- Load only model bundles approved by MODEL-REG-01 for the requested visual
  task class and asset permission class.
- Keep raw private image content out of Control-plane logs and receipts.
- Preserve deterministic seeds or execution settings when an audit-grade visual
  run needs replayability.
- Return verification summaries and artifact hashes to the kernel.
- Support cancellation and budget checks for long visual jobs.

## 5. Non-responsibilities

- Choosing whether the task is allowed.
- Deciding approval corridor outcomes.
- Promoting visual memory.
- Rewriting Done Contracts.
- Treating backend logs as Goni receipts.

## 6. Invariants

- The runtime rejects undeclared workflow hashes.
- The runtime rejects model bundle IDs that are not eligible for the requested
  visual task class.
- Outputs are content-addressed before memory update or export.
- Audit-grade visual runs preserve enough settings to support trace replay when
  the backend can run deterministically.
- Backend workflow logs are diagnostic only; canonical receipts are emitted by
  the Goni kernel.

## 7. Upstream

- [Visual Intelligence Plane](/blueprint/30-specs/visual-intelligence-plane.md)
- [Model bundle registry governance](/blueprint/30-specs/model-registry.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)

## 8. Downstream

- [Receipts](/blueprint/30-specs/receipts.md)
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)

## 9. Adjacent

- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Vector DB](/blueprint/software/30-components/vecdb.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
