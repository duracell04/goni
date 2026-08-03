---
id: GONI-SPEC-881A3FD3FC21
title: 3. Resource scopes
type: specification
status: draft
implementation_state: specified_only
proposition: 'Minimum scope types: fs_scope: path-prefix allowlist with mode constraints, net_scope: domain/method/port allowlist, api_scope: connector method allowlist, state_scope: mutable table/object IDs.'
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
  heading: 3. Resource scopes
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 3. Resource scopes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Resource scopes

Minimum scope types:
- `fs_scope`: path-prefix allowlist with mode constraints,
- `net_scope`: domain/method/port allowlist,
- `api_scope`: connector method allowlist,
- `state_scope`: mutable table/object IDs.

Scope matching is exact or prefix-bounded; wildcards require explicit policy
approval and must be highlighted in decision output.
