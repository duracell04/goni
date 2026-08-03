---
id: GONI-SPEC-8EE0DE25F330
title: 2. Canonical flow
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every meaningful visual task follows this logical flow: The flow may stop early when the requested task is analysis-only, audit-only, or blocked by policy.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 2. Canonical flow
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 2. Canonical flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
