---
id: GONI-PRINCIPLE-F53185B15CF9
title: 6. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**No bypass:** tools cannot be called without a capability token.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 6. Invariants
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 6. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Invariants

- **No bypass:** tools cannot be called without a capability token.
- **Expression is not an effect:** local generation and analysis do not require
  a tool capability solely because of their content.
- **Commit boundary:** durable knowledge changes and external effects require
  the applicable capability regardless of how harmless their content appears.
- **Auditability:** every tool call produces an audit record.
- **Policy mediation:** policy engine is the sole authority for tool approval.
- **Delegation mediation:** autonomy corridor and risk thresholds are evaluated
  before execution.
- **Kernel-owned authority:** external frameworks may surface actions, but they
  do not define authority, corridor outcome, or receipt semantics.
- **Intent traceability:** mutating calls preserve an auditable
  `intent -> plan -> tool intent -> authorized execution` chain.
- **Control-plane traceability:** delegated execution preserves the pre-
  execution references that explain how the Work Order was formed.
- **Transactional safety:** mutating calls are atomic (commit or rollback).
- **Replay safety:** idempotency keys prevent duplicate side effects.
- **Boundary separation:** observation, extraction, memory, actuation, egress,
  and sandbox authority remain separate capability grants for desktop,
  browser, and vision-mediated actions.
