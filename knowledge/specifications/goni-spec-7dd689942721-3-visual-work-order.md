---
id: GONI-SPEC-7DD689942721
title: 3. Visual Work Order
type: specification
status: draft
implementation_state: specified_only
proposition: 'A visual Work Order extends the DELEG-INT-01 WorkOrder with visual-specific metadata stored as summaries and stable refs, not raw private content: The canonical WorkOrders table remains the storage anchor.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 3. Visual Work Order
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 3. Visual Work Order

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
