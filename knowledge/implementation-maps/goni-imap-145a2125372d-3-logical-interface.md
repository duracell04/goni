---
id: GONI-IMAP-145A2125372D
title: 3. Logical interface
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The concrete API may differ, but implementations must preserve the logical fields needed by VIS-01 and REC-01.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/visual-runtime.md
  heading: 3. Logical interface
  revision: 4d6a56dfeb55430356f9e72b203b5df766df28e8
---

# 3. Logical interface

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
