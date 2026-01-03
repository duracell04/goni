# ITCR-01 - Inference-Time Compute Reasoning
Status: Draft (normative target)

This spec defines inference-time compute reasoning (ITCR) as a bounded,
interrupt-driven reasoning service. ITCR is a compute allocation policy over a
reasoning procedure, not a model architecture. It trades extra latency/energy
for reduced error on multi-step planning, verification, and constraint-heavy
tasks.

## 1. Scope and intent

ITCR is invoked only when escalation criteria are met. It operates as a burst
service that verifies and repairs proposals rather than replacing them by
default. The kernel remains the sole authority for side effects (SS-01).

Non-goals:
- continuous deliberation as a default control loop,
- bypassing policy or validation gates,
- unbounded search or reasoning depth.

## 2. Stages and roles

The ITCR cascade is an asymmetric multi-stage pipeline:

Stage 0 - Continuous state maintenance (low power)
- Encoders + predictor update latent state S_core and facts F_sparse.
- Surprisal, risk, and goal conflict signals are computed.

Stage 1 - Proposal generation (cheap proposer)
- A small local model or heuristic planner emits a candidate plan and artifacts.
- Outputs are expected to be fast and fallible.

Stage 2 - Escalation policy (interrupt controller)
- The scheduler raises an ITCR interrupt only when expected value exceeds cost.
- Triggers are explicit predicates over state and request metadata.
- Hysteresis and rate limits prevent oscillation.

Stage 3 - ITCR reasoner/verifier (high power burst)
- The reasoner validates, scores, and repairs proposals.
- Search over candidates is bounded and budgeted.
- Verification is preferred over full regeneration.

Stage 4 - Commit under governance
- Execution only occurs after policy validation and schema checks.
- Tool calls are mediated by capability tokens and audit envelopes.

## 3. Escalation triggers (predicate form)

Triggers are evaluated by the scheduler (SCHED-01) and policy engine (SS-01).
Examples:

- Uncertainty: D(S_t, S_hat_t) > theta_u
- Goal conflict: C(F_t) == true
- Stakes: intent implies irreversible side effects
- Verification failure: proposer fails static checks
- Deadline risk: predicted schedule breach

Escalation must include hysteresis and per-window rate limits.

## 4. ITCR budgets (hard limits)

Every ITCR episode executes within explicit budgets:

- max_wall_time_ms
- max_candidate_expansions
- max_planning_depth
- max_tool_planning_depth
- max_tokens_total

Budget violations terminate the episode with a failed or partial verdict.

## 5. Candidate evaluation and repair loop

ITCR is a propose -> check -> revise loop:

1) Propose: produce one or more candidate plans.
2) Check: score candidates using auxiliary criteria beyond next-token likelihood
   (validators, rule checks, verifier models, constraint evaluation).
3) Revise: repair or resample when checks fail.

The loop exits early if a candidate satisfies all checks or if budgets expire.

## 6. Interfaces (logical)

ITCR consumes a bounded "decision packet":

- state_snapshot_id
- selected evidence with provenance
- proposer artifacts (plan, tool calls, drafts)
- active constraints and capability scopes
- budgets and QoS class

Outputs include:

- selected or repaired plan
- verification reports and constraint outcomes
- updated confidence and risk metadata

## 7. Invariants

- ITCR is interrupt-driven and admission-controlled (SCHED-01).
- No ITCR output bypasses SS-01 validation or TOOL-01 envelopes.
- All ITCR activity is auditable and tied to a state snapshot.
- Budgets are enforced for time, search, and tool-planning depth.

## 8. Related specs

- docs/specs/scheduler-and-interrupts.md
- docs/specs/symbolic-substrate.md
- docs/specs/agent-definition.md
- docs/specs/tool-capability-api.md
- docs/specs/latent-state-contract.md
