---
id: GONI-SPEC-8BAE7731E0CA
title: 7. Minimal constraint payload (v1)
type: specification
status: draft
implementation_state: specified_only
proposition: Constraint values in F_sparse are versioned JSON objects.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 7. Minimal constraint payload (v1)
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 7. Minimal constraint payload (v1)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Minimal constraint payload (v1)

Constraint values in F_sparse are versioned JSON objects. Minimal shape:

{
  "v": 1,
  "effect": "allow" | "deny",
  "subject": {"tool_id": "..."} | {"capability": "..."} | null,
  "when": {"op": "all|any|not|eq|in|exists|missing|schema", "args": [...]},
  "on_fail": "block" | "ask" | "defer",
  "reason": "optional text"
}

Evaluation rules:
- when/op predicates read from F_sparse by key or key.field path.
- schema predicates validate tool args or artifacts against a known schema.
- absent or malformed constraints are treated as deny by default.

Example constraints (JSON values stored under constraint.* keys):

- constraint.no_send_email:
  {"v":1,"effect":"deny","subject":{"tool_id":"email.send"},"on_fail":"block"}
- constraint.requires_source:
  {"v":1,"effect":"deny","subject":{"tool_id":"fs.write"},
   "when":{"op":"missing","args":["fact.source_ref"]},"on_fail":"ask"}
