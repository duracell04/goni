---
id: GONI-PRINCIPLE-1630A157EA03
title: 5. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**LLM is an interrupt:** solver calls are admission-controlled and budgeted.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 5. Invariants
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 5. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Invariants

- **LLM is an interrupt:** solver calls are admission-controlled and budgeted.
- **No hidden queues:** all work enters the scheduler.
- **Wake hysteresis:** the kernel enforces cooldowns and rate limits.
- **Budget enforcement:** CPU/GPU time, disk writes, and solver calls are capped.
- **Routing remains external:** runtime backends may expose speculative decoding
  controls, but routing, escalation, and acceptance policy remain in the
  router/control plane.
