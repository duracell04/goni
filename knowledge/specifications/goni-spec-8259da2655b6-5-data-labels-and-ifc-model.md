---
id: GONI-SPEC-8259DA2655B6
title: 5. Data labels and IFC model
type: specification
status: draft
implementation_state: specified_only
proposition: 'The policy language defines labels for protected data classes, for example: public internal user_private sensitive_pii secret Every mediated operation evaluates: source label set, destination label constraints, declassification requirements.'
domains:
- kernel
- policy
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md
  heading: 5. Data labels and IFC model
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 5. Data labels and IFC model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Data labels and IFC model

The policy language defines labels for protected data classes, for example:
- `public`
- `internal`
- `user_private`
- `sensitive_pii`
- `secret`

Every mediated operation evaluates:
- source label set,
- destination label constraints,
- declassification requirements.
