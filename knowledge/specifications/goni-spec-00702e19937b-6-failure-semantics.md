---
id: GONI-SPEC-00702E19937B
title: 6. Failure semantics
type: specification
status: draft
implementation_state: specified_only
proposition: 'When validation fails, the kernel must choose one of: block: deny action and record the failure in AuditRecords.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 6. Failure semantics
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 6. Failure semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Failure semantics

When validation fails, the kernel must choose one of:
- block: deny action and record the failure in AuditRecords.
- ask: raise a user confirmation interrupt and defer execution.
- defer: request more evidence or schedule later.
- degrade: reduce capability/budget or switch modes (SCHED-01).

Failures never execute tool calls and never commit state changes.
