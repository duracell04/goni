---
id: GONI-SYNTHESIS-4FED0137EF33
title: 3) Tool execution
type: synthesis
status: draft
implementation_state: specified_only
proposition: time-to-tool-plan tool call duration + retries/backoff tool result parse/validation time success rate and partial failure rate idempotency violations capability overreach rate (attempted calls outside token scope) undo success rate (if reversible actions exist) approval rate, time-to-approval, user edit distance (if HITL)
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 3) Tool execution
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 3) Tool execution

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3) Tool execution

- time-to-tool-plan
- tool call duration + retries/backoff
- tool result parse/validation time
- success rate and partial failure rate
- idempotency violations
- capability overreach rate (attempted calls outside token scope)
- undo success rate (if reversible actions exist)
- approval rate, time-to-approval, user edit distance (if HITL)
