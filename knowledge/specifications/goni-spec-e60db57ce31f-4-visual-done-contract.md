---
id: GONI-SPEC-E60DB57CE31F
title: 4. Visual Done Contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'A visual Done Contract extends DELEG-INT-01 DoneContract with visual completion criteria: Evidence and legal visual tasks MUST use audit_grade work-quality mode.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 4. Visual Done Contract
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 4. Visual Done Contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
