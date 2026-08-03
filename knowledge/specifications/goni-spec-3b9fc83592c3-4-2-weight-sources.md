---
id: GONI-SPEC-3B9FC83592C3
title: 4.2 Weight Sources
type: specification
status: draft
implementation_state: specified_only
proposition: 'Graph weight has three inspectable sources: explicit is set by the principal or an authorized user action.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 4.2 Weight Sources
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 4.2 Weight Sources

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.2 Weight Sources

Graph weight has three inspectable sources:

```yaml
weight:
  explicit: 0.9
  inferred: 0.62
  reinforced: 0.74
  final: 0.83
```

`explicit` is set by the principal or an authorized user action. `inferred` is
created by parsers, models, importers, or deterministic rules. `reinforced` is
updated from accepted repeated use, citations, corrections, or confirmations.
`final` is derived from these components by the scoring policy and MUST remain
auditable. A user-set explicit weight MAY dominate inferred and reinforced
weights when policy allows.
