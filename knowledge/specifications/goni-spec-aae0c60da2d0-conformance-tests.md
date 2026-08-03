---
id: GONI-SPEC-AAE0C60DA2D0
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Any request lacking an allow rule is denied.
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
  heading: Conformance tests
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- Any request lacking an allow rule is denied.
- Label downgrade without matching declassification rule is denied.
- Budget exhaustion yields deny with machine-readable cause.
- Every allow/deny decision is linked to a receipt via `policy_hash` and
  decision reference.
