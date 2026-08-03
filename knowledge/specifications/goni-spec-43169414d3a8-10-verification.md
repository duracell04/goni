---
id: GONI-SPEC-43169414D3A8
title: 10. Verification
type: specification
status: draft
implementation_state: specified_only
proposition: 'Before presenting generated or edited visual work as final, Goni SHOULD evaluate the output against the Done Contract using task-appropriate checks: text/OCR legibility layout hierarchy object preservation mask accuracy style consistency brand consistency source-faithfulness private-data leakage license and rights compatibility'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 10. Verification
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 10. Verification

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
