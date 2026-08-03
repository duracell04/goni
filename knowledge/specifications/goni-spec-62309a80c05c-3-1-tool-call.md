---
id: GONI-SPEC-62309A80C05C
title: 3.1 `tool_call`
type: specification
status: draft
implementation_state: specified_only
proposition: 'tool_call(cap_handle, action, args_ref, tx_id, idempotency_key) -> outcome Outcome envelope includes: status (ok | denied | error | cancelled) result_ref (or error ref) receipt_ref (mandatory) policy_decision_ref'
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 3.1 `tool_call`
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.1 `tool_call`

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 `tool_call`

`tool_call(cap_handle, action, args_ref, tx_id, idempotency_key) -> outcome`

Outcome envelope includes:
- `status` (`ok` | `denied` | `error` | `cancelled`)
- `result_ref` (or error ref)
- `receipt_ref` (mandatory)
- `policy_decision_ref`
