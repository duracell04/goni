---
id: GONI-SPEC-3232ED52F1D1
title: 5. Mandatory receipt semantics
type: specification
status: draft
implementation_state: specified_only
proposition: Every tool_call, commit_tx, and rollback_tx MUST emit exactly one receipt.
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
  heading: 5. Mandatory receipt semantics
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 5. Mandatory receipt semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Mandatory receipt semantics

Every `tool_call`, `commit_tx`, and `rollback_tx` MUST emit exactly one receipt.

Receipt minimum:
- `trace_id`
- `span_id`
- `job_id`
- `tx_id`
- `capability_id`
- `policy_decision`
- `action_type`
- `timestamp`

See `blueprint/30-specs/receipts.md` for canonical receipt fields.
