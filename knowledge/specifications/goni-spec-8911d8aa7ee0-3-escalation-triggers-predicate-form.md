---
id: GONI-SPEC-8911D8AA7EE0
title: 3. Escalation triggers (predicate form)
type: specification
status: draft
implementation_state: specified_only
proposition: Triggers are evaluated by the scheduler (SCHED-01) and policy engine (SS-01).
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 3. Escalation triggers (predicate form)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 3. Escalation triggers (predicate form)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Escalation triggers (predicate form)

Triggers are evaluated by the scheduler (SCHED-01) and policy engine (SS-01).
Examples:

- Uncertainty: D(S_t, S_hat_t) > theta_u
- Goal conflict: C(F_t) == true
- Stakes: intent implies irreversible side effects
- Verification failure: proposer fails static checks
- Deadline risk: predicted schedule breach

Escalation must include hysteresis and per-window rate limits.
