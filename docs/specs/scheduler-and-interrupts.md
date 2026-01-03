# SCHED-01 - Scheduler and Interrupts
DOC-ID: SCHED-01
Status: Draft (normative target)

This spec defines when and how the kernel escalates work from continuous,
low-power cognition to expensive solver/LLM interrupts.

## 1. Interrupt triggers

Interrupts are raised on:

- `surprisal > threshold`
- `goal_conflict` detected by predictor
- explicit user intent
- policy deadlines or compliance conditions

Interrupts are routed through the scheduler and never call the solver directly.
Admission is part of the SS-01 arbitration contract.

## 2. Hysteresis and wake control

To prevent thrash:

- minimum cooldown between solver wakes,
- max solver wakes per time window,
- rising and falling thresholds for surprisal.

## 2.1 ITCR budgets

When an interrupt escalates into an ITCR episode (ITCR-01), the scheduler
attaches hard budgets:

- max_wall_time_ms
- max_candidate_expansions
- max_planning_depth
- max_tool_planning_depth
- max_tokens_total

Budget exhaustion terminates the episode and is recorded in audit logs.

## 3. QoS classes and preemption

Work is scheduled into classes:

- interactive (user-facing),
- background (indexing, maintenance),
- maintenance (compaction, audits).

Class priorities are enforced by the scheduler (MaxWeight policy).

## 4. Degradation modes

The kernel exposes explicit modes:

- Eco
- Normal
- Boost
- Thermal throttle
- Offline-safe

Mode changes adjust budgets, wake rates, and compaction thresholds.

## 5. Invariants

- **LLM is an interrupt:** solver calls are admission-controlled and budgeted.
- **No hidden queues:** all work enters the scheduler.
- **Wake hysteresis:** the kernel enforces cooldowns and rate limits.
- **Budget enforcement:** CPU/GPU time, disk writes, and solver calls are capped.

## 6. Audit fields

Interrupt decisions and solver wakes are recorded with:

- `agent_id` (if an agent requested the interrupt)
- `policy_hash`
- `state_snapshot_id`
- `provenance`

## 7. Related specs

- `docs/specs/latent-state-contract.md`
- `docs/specs/tool-capability-api.md`
- `docs/specs/symbolic-substrate.md`
- `docs/specs/itcr.md`
