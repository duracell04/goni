---
id: GONI-SPEC-7AD358800BEF
title: 2.7 ReconstructionPreview
type: specification
status: draft
implementation_state: specified_only
proposition: 'For user-facing previews, approvals, or operator inspection, the runtime MUST be able to produce a ReconstructionPreview containing: goal done assumptions risk question For audit_grade work, the preview SHOULD also expose: scope_checked_or_planned missing_evidence claim_strength_limit The preview is derived from the Work Order and policy state.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.7 ReconstructionPreview
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.7 ReconstructionPreview

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.7 ReconstructionPreview

For user-facing previews, approvals, or operator inspection, the runtime MUST
be able to produce a `ReconstructionPreview` containing:

- `goal`
- `done`
- `assumptions`
- `risk`
- `question`

For `audit_grade` work, the preview SHOULD also expose:

- `scope_checked_or_planned`
- `missing_evidence`
- `claim_strength_limit`

The preview is derived from the Work Order and policy state. It must not rely
on UI-only shadow state.
