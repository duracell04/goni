---
id: GONI-PRINCIPLE-197860126029
title: 2) Turn invariants into real tests
type: principle
status: draft
implementation_state: specified_only
proposition: 'Replace placeholders in blueprint/tests/invariants/ with real checks: TXT boundary lint receipt chain verification default-deny policy behavior egress allowlist enforcement (deny by default) Wire them into CI (fast gates on PRs).'
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 2) Turn invariants into real tests
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 2) Turn invariants into real tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2) Turn invariants into real tests
- Replace placeholders in `blueprint/tests/invariants/` with real checks:
  - TXT boundary lint
  - receipt chain verification
  - default-deny policy behavior
  - egress allowlist enforcement (deny by default)
- Wire them into CI (fast gates on PRs).
