---
id: GONI-SPEC-55B7F8940C9D
title: 3.3 `commit_tx`
type: specification
status: draft
implementation_state: specified_only
proposition: commit_tx(tx_id) -> {status, receipt_ref} Applies staged effects if preconditions and policy still hold.
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
  heading: 3.3 `commit_tx`
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.3 `commit_tx`

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 `commit_tx`

`commit_tx(tx_id) -> {status, receipt_ref}`

Applies staged effects if preconditions and policy still hold.
