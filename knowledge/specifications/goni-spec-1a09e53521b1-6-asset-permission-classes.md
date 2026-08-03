---
id: GONI-SPEC-1A09E53521B1
title: 6. Asset permission classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'Visual tasks carry an asset permission class because images often contain identity, private context, rights, or evidence value: | Class | Default posture | | public_reference | Low-risk analysis or generation reference when rights are known.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 6. Asset permission classes
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 6. Asset permission classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Asset permission classes

Visual tasks carry an asset permission class because images often contain
identity, private context, rights, or evidence value:

| Class | Default posture |
| --- | --- |
| `public_reference` | Low-risk analysis or generation reference when rights are known. |
| `project_owned` | Allowed for project workflows under project policy. |
| `private_screenshot` | Requires minimization, leakage checks, and local-first routing by default. |
| `person_identifying` | Requires stricter approval for identity-preserving edits, face use, or export. |
| `brand_sensitive` | Requires brand/right checks and imitation limits. |
| `legal_evidence` | Audit-grade only; transformations must be annotation, comparison, or reversible preparation unless explicitly approved. |

Policy may add narrower classes. More sensitive classes raise the approval
corridor, assurance floor, and receipt tier.
