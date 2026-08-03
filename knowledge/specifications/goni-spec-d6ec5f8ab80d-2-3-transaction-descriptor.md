---
id: GONI-SPEC-D6EC5F8AB80D
title: 2.3 Transaction descriptor
type: specification
status: draft
implementation_state: specified_only
proposition: 'Required fields: tx_id operation_id idempotency_key (required for mutating calls) preconditions'
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
  heading: 2.3 Transaction descriptor
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 2.3 Transaction descriptor

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.3 Transaction descriptor

Required fields:
- `tx_id`
- `operation_id`
- `idempotency_key` (required for mutating calls)
- `preconditions`
