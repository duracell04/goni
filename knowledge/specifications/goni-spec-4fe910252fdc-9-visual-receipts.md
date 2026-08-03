---
id: GONI-SPEC-4FE910252FDC
title: 9. Visual receipts
type: specification
status: draft
implementation_state: specified_only
proposition: VIS-01 extends REC-01 with a visual_basis object.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 9. Visual receipts
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 9. Visual receipts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
