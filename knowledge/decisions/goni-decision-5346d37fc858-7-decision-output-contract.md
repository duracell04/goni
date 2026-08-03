---
id: GONI-DECISION-5346D37FC858
title: 7. Decision output contract
type: decision
status: draft
implementation_state: specified_only
proposition: 'Every policy decision returns: decision (allow | deny | allow_with_constraints) matched_rules capability_id budget_delta label_flow_decision declassification_ref (if applied) policy_hash Decision output is mandatory input to receipt emission.'
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
  heading: 7. Decision output contract
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 7. Decision output contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Decision output contract

Every policy decision returns:
- `decision` (`allow` | `deny` | `allow_with_constraints`)
- `matched_rules`
- `capability_id`
- `budget_delta`
- `label_flow_decision`
- `declassification_ref` (if applied)
- `policy_hash`

Decision output is mandatory input to receipt emission.
