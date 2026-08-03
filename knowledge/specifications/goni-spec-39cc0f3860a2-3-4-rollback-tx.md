---
id: GONI-SPEC-39CC0F3860A2
title: 3.4 `rollback_tx`
type: specification
status: draft
implementation_state: specified_only
proposition: rollback_tx(tx_id, reason) -> {status, receipt_ref} Rolls back staged operations and records failure/deny semantics.
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
  heading: 3.4 `rollback_tx`
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.4 `rollback_tx`

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 `rollback_tx`

`rollback_tx(tx_id, reason) -> {status, receipt_ref}`

Rolls back staged operations and records failure/deny semantics.
