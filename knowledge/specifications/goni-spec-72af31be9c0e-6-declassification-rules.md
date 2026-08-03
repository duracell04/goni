---
id: GONI-SPEC-72AF31BE9C0E
title: 6. Declassification rules
type: specification
status: draft
implementation_state: specified_only
proposition: 'A declassification rule must include: authorized actor class, allowed downgrade path (e.g., secret -> internal), justification code, optional approval requirement.'
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
  heading: 6. Declassification rules
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 6. Declassification rules

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Declassification rules

A declassification rule must include:
- authorized actor class,
- allowed downgrade path (e.g., `secret -> internal`),
- justification code,
- optional approval requirement.

Any declassification emits a dedicated receipt entry with justification.
