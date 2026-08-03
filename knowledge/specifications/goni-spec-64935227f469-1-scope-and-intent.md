---
id: GONI-SPEC-64935227F469
title: 1. Scope and intent
type: specification
status: draft
implementation_state: specified_only
proposition: ITCR is invoked only when escalation criteria are met.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 1. Scope and intent
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 1. Scope and intent

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope and intent

ITCR is invoked only when escalation criteria are met. It operates as a burst
service that verifies and repairs proposals rather than replacing them by
default. The kernel remains the sole authority for side effects (SS-01).

Non-goals:
- continuous deliberation as a default control loop,
- bypassing policy or validation gates,
- unbounded search or reasoning depth.
