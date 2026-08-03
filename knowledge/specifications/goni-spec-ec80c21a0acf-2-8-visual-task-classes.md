---
id: GONI-SPEC-EC80C21A0ACF
title: 2.8 Visual task classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'VIS-01 defines governed visual task classes that use the same WorkOrder and DoneContract objects: visual.logo_refinement visual.image_edit visual.design_board visual.diagram visual.screenshot_audit visual.product_visual visual.evidence_annotation Visual task classes MUST carry source asset refs, asset permission class, visual constraints, output target, and verification requirements through the'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.8 Visual task classes
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.8 Visual task classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.8 Visual task classes

VIS-01 defines governed visual task classes that use the same WorkOrder and
DoneContract objects:

- `visual.logo_refinement`
- `visual.image_edit`
- `visual.design_board`
- `visual.diagram`
- `visual.screenshot_audit`
- `visual.product_visual`
- `visual.evidence_annotation`

Visual task classes MUST carry source asset refs, asset permission class,
visual constraints, output target, and verification requirements through the
Work Order. They MUST NOT store raw private screenshots, raw image binaries, or
unbounded OCR text in Control-plane fields.
