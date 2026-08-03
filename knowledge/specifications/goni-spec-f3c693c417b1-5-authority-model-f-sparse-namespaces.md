---
id: GONI-SPEC-F3C693C417B1
title: 5. Authority model (F_sparse namespaces)
type: specification
status: draft
implementation_state: specified_only
proposition: F_sparse is a map of namespaced keys.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 5. Authority model (F_sparse namespaces)
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 5. Authority model (F_sparse namespaces)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Authority model (F_sparse namespaces)

F_sparse is a map of namespaced keys. Authority is enforced by policy:

- policy.*     kernel-owned; only the policy engine can write.
- constraint.* kernel-owned; immutable except via policy updates.
- goal.*       agent-proposed; must pass validation before commit.
- fact.*       encoder/tool-derived; must pass validation before commit.

All writes occur through StateDeltas. Direct mutation is forbidden.
