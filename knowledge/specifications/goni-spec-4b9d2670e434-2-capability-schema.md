---
id: GONI-SPEC-4B9D2670E434
title: 2. Capability schema
type: specification
status: draft
implementation_state: specified_only
proposition: 'Each capability grant includes: capability_id subject (agent/tool identity) resource_scope (fs/net/api/object set) action_set (read/write/execute/send/etc.) budget_caps (calls, bytes, time, spend units) valid_from, expires_at revocation_ref Capabilities are non-transferrable unless explicit delegation rule exists.'
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
  heading: 2. Capability schema
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 2. Capability schema

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Capability schema

Each capability grant includes:
- `capability_id`
- `subject` (agent/tool identity)
- `resource_scope` (fs/net/api/object set)
- `action_set` (read/write/execute/send/etc.)
- `budget_caps` (calls, bytes, time, spend units)
- `valid_from`, `expires_at`
- `revocation_ref`

Capabilities are non-transferrable unless explicit delegation rule exists.
