---
id: SCHED-01
type: SPEC
status: specified_only
---
# SCHED-01 - Scheduler and Interrupts
DOC-ID: SCHED-01
Status: Specified only / roadmap

This spec defines when and how the kernel escalates work from continuous,
low-power cognition to expensive solver/LLM interrupts.

## 1. Interrupt triggers

Interrupts are raised on:

- `surprisal > threshold`
- `goal_conflict` detected by predictor
- explicit user intent
- policy deadlines or compliance conditions
- missing information that materially changes safe delegated execution
- unresolved objective ambiguity that requires `co_creation`

Interrupts are routed through the scheduler and never call the solver directly.
Admission is part of the SS-01 arbitration contract.

## 1.1 Clarification interrupts

Clarification is a bounded interrupt subtype for delegation engineering.

Clarification interrupts are allowed only when:

- a missing answer would change corridor or threshold outcome,
- a missing answer would change tool choice or counterparty/action target,
- a side effect is irreversible or costly enough that assumptions are
  insufficient,
- policy explicitly requires a user answer for the current task class.

Clarification interrupts are not allowed when the missing information can be
derived from active policy, retrieved context, prior approvals, or stable task
defaults. In those cases the runtime must proceed under surfaced assumptions or
block/escalate the action.

The scheduler MUST track clarification budgets and cooldowns separately from
general solver wakes so an over-questioning agent cannot degrade the operator
experience.

### 1.2 Co-creation interrupts

Co-creation is a separate interrupt subtype for unresolved objective choice.

Co-creation interrupts are allowed only when:

- two or more materially different objectives remain plausible,
- selecting among them would define the user's goal rather than execute it,
- policy does not permit silent defaulting for the current task class.

Co-creation interrupts are not allowed for merely missing factual details. In
those cases the runtime must remain in delegation mode and choose among
`assume`, `ask_decisive`, or `block`.

The scheduler MUST track co-creation interrupts separately from clarification
interrupts so goal ambiguity is not conflated with missing-parameter lookup.

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

## 3.1 Delegation escalation lanes

Delegated execution must route through explicit lanes:

- `autonomous`: executes within active corridor and risk threshold,
- `review`: deferred/batched review for soft-gate actions,
- `blocked`: denied by no-go policy or risk overflow,
- `escalated`: requires immediate user decision.

The scheduler is responsible for fairness and latency bounds across these lanes.

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
- `task_class`
- `autonomy_mode`
- `interaction_mode`
- `risk_score`
- `clarification_status`
- `clarification_decision`
- `work_order_id`
- `delegation_outcome`

## 7. Related specs

- [latent-state-contract.md](/blueprint/30-specs/latent-state-contract.md)
- [delegation-interface.md](/blueprint/30-specs/delegation-interface.md)
- [tool-capability-api.md](/blueprint/30-specs/tool-capability-api.md)
- [symbolic-substrate.md](/blueprint/30-specs/symbolic-substrate.md)
- [itcr.md](/blueprint/30-specs/itcr.md)
- [delegation-and-autonomy.md](/blueprint/30-specs/delegation-and-autonomy.md)

## 8. Upstream
- [Job contract](/blueprint/30-specs/job.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)

## 9. Downstream
- [Orchestrator](/blueprint/software/30-components/orchestrator.md)
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [ITCR](/blueprint/30-specs/itcr.md)

## 10. Adjacent
- [Agent definition](/blueprint/30-specs/agent-definition.md)
- [Symbolic substrate](/blueprint/30-specs/symbolic-substrate.md)
- [System map](/blueprint/docs/00-system-map.md)

## Conformance tests
- clarification interrupts must be suppressed when the answer is derivable from
  policy or retrieved context
- clarification interrupts must be raised when missing information materially
  changes corridor, risk, or irreversible side effects
- co-creation interrupts must be raised when goal ambiguity is genuine and must
  be suppressed for mere factual omission
- clarification budget exhaustion must lead to surfaced assumptions,
  escalation, or blocking rather than repeated questioning
- scheduler audit fields must record `interaction_mode`,
  `clarification_decision`, `clarification_status`, and
  `delegation_outcome`






