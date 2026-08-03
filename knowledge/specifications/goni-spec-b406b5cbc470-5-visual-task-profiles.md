---
id: GONI-SPEC-B406B5CBC470
title: 5. Visual task profiles
type: specification
status: draft
implementation_state: specified_only
proposition: Visual tasks are not interchangeable.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 5. Visual task profiles
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 5. Visual task profiles

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
