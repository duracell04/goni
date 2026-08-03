---
id: GONI-SPEC-1D4A3D0323EB
title: 2.1 Transactional tool semantics (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: 'All mutating tool calls MUST execute as mediated transactions: **Prepare/precondition check:** validate capability token and policy hash, validate precondition_refs against current state/version, and reserve budget according to policy.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 2.1 Transactional tool semantics (normative)
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 2.1 Transactional tool semantics (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1 Transactional tool semantics (normative)

All mutating tool calls MUST execute as mediated transactions:

- **Prepare/precondition check:** validate capability token and policy hash,
  validate `precondition_refs` against current state/version, and reserve
  budget according to policy.
- **Authorization decision:** record whether execution is autonomous, queued for
  review, escalated, or denied, and require approval or two-phase commit for
  irreversible actions.
- **Commit:** apply side effects atomically, append state delta(s), and emit
  audit record + receipt with resulting hashes.
- **Rollback:** on policy rejection, failed preconditions, or commit failure,
  no partial side effects may remain; emit an auditable failure result
  (`tx_outcome = rolled_back`).

Replay and idempotency rules:
- mutating calls MUST include `idempotency_key`,
- repeated `(tool_id, idempotency_key, args_hash, capability_token_id)` MUST
  return the original outcome without duplicate side effects,
- stale/invalid replay attempts MUST fail closed and remain auditable.
