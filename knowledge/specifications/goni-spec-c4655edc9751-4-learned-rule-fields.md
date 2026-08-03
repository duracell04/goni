---
id: GONI-SPEC-C4655EDC9751
title: 4. Learned-rule fields
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every candidate or accepted learned rule MUST preserve: scope: global | project | channel | recipient | task_class | session.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 4. Learned-rule fields
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 4. Learned-rule fields

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Learned-rule fields

Every candidate or accepted learned rule MUST preserve:

- `scope`: `global | project | channel | recipient | task_class | session`.
- `confidence`: numeric confidence in the inferred rule.
- `evidence_count`: number of supporting deltas or actions.
- `contradiction_count`: number of conflicting deltas or actions.
- `decay_policy`: expiry, review, or reinforcement behavior.
- `review_status`: `pending | accepted | rejected | limited`.
- `source_refs`: chunk, prompt, receipt, diff, or interaction refs.
- `memory_diff_refs`: memory or state delta refs caused by acceptance.
- `regression_test_refs`: replay or harness tests attached to the rule.

Accepted learned preferences SHOULD be stored as `MemoryEntries` with:

- `kind = hypothesis | preference | derived`
- `memory_class = procedural | policy | project | relational`

Scope, confidence, evidence count, contradiction count, decay policy, review
status, learning receipt refs, and regression refs live in `value` or
`provenance` until a later schema version introduces first-class fields.
